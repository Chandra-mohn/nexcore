use std::collections::HashMap;

use cobol_transpiler::ast::{
    CallStatement, CobolProgram, DataEntry, FileDescription, Literal, OpenMode, Operand,
    ProcedureDivision, Sentence, Statement,
};

use super::edge::{Edge, EdgeKind};
use super::node::{Node, NodeId, NodeKind, PropValue};
use super::CodeGraph;
use crate::error::IntelResult;

/// Build a `CodeGraph` from one or more parsed COBOL programs.
#[derive(Debug)]
pub struct GraphBuilder {
    graph: CodeGraph,
    /// Track copybook nodes already created (dedup across programs).
    copybook_ids: HashMap<String, NodeId>,
    /// Track file nodes already created (dedup across programs).
    file_ids: HashMap<String, NodeId>,
    /// Track table nodes already created (dedup across programs).
    table_ids: HashMap<String, NodeId>,
}

impl GraphBuilder {
    pub fn new() -> Self {
        Self {
            graph: CodeGraph::new(),
            copybook_ids: HashMap::new(),
            file_ids: HashMap::new(),
            table_ids: HashMap::new(),
        }
    }

    /// Add a parsed COBOL program to the graph.
    pub fn add_program(
        &mut self,
        program: &CobolProgram,
        copybooks_used: &[String],
    ) -> IntelResult<()> {
        let prog_name = &program.program_id;

        // Count LOC from procedure division
        let paragraph_count = program
            .procedure_division
            .as_ref()
            .map_or(0, count_paragraphs);

        let prog_id = self.graph.add_node(
            Node::new(NodeKind::Program, prog_name)
                .with_property("paragraph_count", PropValue::from(paragraph_count as u64)),
        );

        // Copybooks
        for cb_name in copybooks_used {
            let cb_id = self.ensure_copybook(cb_name);
            self.graph
                .add_edge(prog_id, cb_id, Edge::new(EdgeKind::Uses));
            self.graph.index_copybook_usage(cb_name, prog_id);
        }

        // File descriptions
        if let Some(dd) = &program.data_division {
            for fd in &dd.file_section {
                let file_id = self.ensure_file(fd);
                self.graph
                    .add_edge(prog_id, file_id, Edge::new(EdgeKind::Accesses));
                self.graph.index_file_access(&fd.file_name, prog_id);
            }

            // Fields from WORKING-STORAGE
            self.add_fields(&dd.working_storage, prog_name, prog_id, "working-storage");

            // Fields from LINKAGE
            self.add_fields(&dd.linkage, prog_name, prog_id, "linkage");

            // Fields from LOCAL-STORAGE
            self.add_fields(&dd.local_storage, prog_name, prog_id, "local-storage");
        }

        // EXEC SQL tables
        for sql in &program.exec_sql_statements {
            if let Some(table) = extract_table_name(&sql.raw_sql) {
                let table_id = self.ensure_table(&table);
                self.graph.add_edge(
                    prog_id,
                    table_id,
                    Edge::new(EdgeKind::Accesses)
                        .with_property("sql_type", PropValue::from(format!("{:?}", sql.sql_type))),
                );
            }
        }

        // Procedure division: paragraphs, PERFORM, CALL
        if let Some(pd) = &program.procedure_division {
            let mut para_ids: HashMap<String, NodeId> = HashMap::new();

            // Collect all paragraph nodes first
            for section in &pd.sections {
                for para in &section.paragraphs {
                    let para_id = self.add_paragraph(&para.name, prog_name, prog_id, &section.name);
                    para_ids.insert(para.name.clone(), para_id);
                }
            }
            for para in &pd.paragraphs {
                let para_id = self.add_paragraph(&para.name, prog_name, prog_id, "");
                para_ids.insert(para.name.clone(), para_id);
            }

            // Now walk statements for PERFORM and CALL edges
            for section in &pd.sections {
                for para in &section.paragraphs {
                    if let Some(para_id) = para_ids.get(&para.name) {
                        self.walk_paragraph_statements(
                            *para_id,
                            prog_id,
                            prog_name,
                            &para.sentences,
                            &para_ids,
                        );
                    }
                }
            }
            for para in &pd.paragraphs {
                if let Some(para_id) = para_ids.get(&para.name) {
                    self.walk_paragraph_statements(
                        *para_id,
                        prog_id,
                        prog_name,
                        &para.sentences,
                        &para_ids,
                    );
                }
            }
        }

        Ok(())
    }

    /// Store metadata on a copybook node for data complexity analysis.
    ///
    /// Call this when copybook content is available (e.g., during a separate
    /// copybook scanning pass). The intelligence `DataComplexityPass` uses
    /// these properties to compute `copybook_complexity` and `is_monster`.
    pub fn add_copybook_metadata(
        &mut self,
        name: &str,
        field_count: u64,
        redefines_depth: u64,
        occurs_count: u64,
        record_length: u64,
    ) {
        let cb_id = self.ensure_copybook(name);
        if let Some(node) = self.graph.node_mut(cb_id) {
            node.properties
                .set("cb_field_count", PropValue::from(field_count));
            node.properties
                .set("cb_redefines_depth", PropValue::from(redefines_depth));
            node.properties
                .set("cb_occurs_count", PropValue::from(occurs_count));
            node.properties
                .set("cb_record_length", PropValue::from(record_length));
        }
    }

    /// Consume the builder and return the finished graph.
    pub fn build(self) -> CodeGraph {
        self.graph
    }

    fn add_paragraph(
        &mut self,
        name: &str,
        prog_name: &str,
        prog_id: NodeId,
        section: &str,
    ) -> NodeId {
        let mut node = Node::new(NodeKind::Paragraph, name).with_program(prog_name);
        if !section.is_empty() {
            node = node.with_property("section", PropValue::from(section));
        }
        let para_id = self.graph.add_node(node);
        self.graph
            .add_edge(prog_id, para_id, Edge::new(EdgeKind::Contains));
        para_id
    }

    fn add_fields(
        &mut self,
        entries: &[DataEntry],
        prog_name: &str,
        prog_id: NodeId,
        section: &str,
    ) {
        for entry in entries {
            self.add_field_recursive(entry, prog_name, prog_id, section);
        }
    }

    fn add_field_recursive(
        &mut self,
        entry: &DataEntry,
        prog_name: &str,
        prog_id: NodeId,
        section: &str,
    ) {
        if entry.name == "FILLER" {
            // Skip FILLER items -- they have no semantic meaning for queries
            for child in &entry.children {
                self.add_field_recursive(child, prog_name, prog_id, section);
            }
            return;
        }

        let mut node = Node::new(NodeKind::Field, &entry.name)
            .with_program(prog_name)
            .with_property("level", PropValue::from(entry.level as u64))
            .with_property("section", PropValue::from(section));

        if let Some(pic) = &entry.pic {
            node = node.with_property("pic", PropValue::from(pic.raw.clone()));
        }
        if let Some(size) = entry.byte_length {
            node = node.with_property("size_bytes", PropValue::from(size as u64));
        }
        if entry.redefines.is_some() {
            node = node.with_property("redefines", PropValue::from(true));
        }
        if entry.occurs.is_some() || entry.occurs_depending.is_some() {
            node = node.with_property("is_array", PropValue::from(true));
        }

        let _field_id = self.graph.add_node(node);

        for child in &entry.children {
            self.add_field_recursive(child, prog_name, prog_id, section);
        }
    }

    fn walk_paragraph_statements(
        &mut self,
        para_id: NodeId,
        prog_id: NodeId,
        prog_name: &str,
        sentences: &[Sentence],
        para_ids: &HashMap<String, NodeId>,
    ) {
        for sentence in sentences {
            for stmt in &sentence.statements {
                self.walk_statement(para_id, prog_id, prog_name, stmt, para_ids);
            }
        }
    }

    fn walk_statement(
        &mut self,
        para_id: NodeId,
        prog_id: NodeId,
        prog_name: &str,
        stmt: &Statement,
        para_ids: &HashMap<String, NodeId>,
    ) {
        match stmt {
            Statement::Perform(perform) => {
                if let Some(target) = &perform.target {
                    let target_name = target.name.to_uppercase();
                    if let Some(target_id) = para_ids.get(&target_name) {
                        let mut edge = Edge::new(EdgeKind::Performs);
                        if let Some(thru) = &perform.thru {
                            edge = edge.with_property("thru", PropValue::from(thru.clone()));
                        }
                        self.graph.add_edge(para_id, *target_id, edge);
                    }
                }
                // Walk inline PERFORM body
                for body_stmt in &perform.body {
                    self.walk_statement(para_id, prog_id, prog_name, body_stmt, para_ids);
                }
            }

            Statement::Call(call) => {
                self.handle_call(prog_id, prog_name, call);
            }

            // Walk nested statements in control flow
            Statement::If(if_stmt) => {
                for s in &if_stmt.then_body {
                    self.walk_statement(para_id, prog_id, prog_name, s, para_ids);
                }
                for s in &if_stmt.else_body {
                    self.walk_statement(para_id, prog_id, prog_name, s, para_ids);
                }
            }

            Statement::Evaluate(eval) => {
                for when in &eval.when_branches {
                    for s in &when.body {
                        self.walk_statement(para_id, prog_id, prog_name, s, para_ids);
                    }
                }
                for s in &eval.when_other {
                    self.walk_statement(para_id, prog_id, prog_name, s, para_ids);
                }
            }

            // File I/O: track paragraph-level file access with mode
            Statement::Open(open) => {
                for file in &open.files {
                    let mode = match file.mode {
                        OpenMode::Input => "read",
                        OpenMode::Output => "write",
                        OpenMode::IoMode => "read-write",
                        OpenMode::Extend => "append",
                    };
                    self.add_para_file_access(para_id, &file.file_name, mode);
                }
            }
            Statement::Read(_) => {
                // READ uses the file associated with the FD, but we don't have
                // the file name directly in ReadStatement in all cases.
                // The OPEN already establishes the relationship.
            }
            Statement::Write(_) | Statement::Rewrite(_) => {
                // Similarly, WRITE uses the file from the FD record.
                // OPEN already captures the mode.
            }

            // Other statements: no graph edges needed at this layer
            _ => {}
        }
    }

    fn handle_call(&mut self, prog_id: NodeId, _prog_name: &str, call: &CallStatement) {
        let target_name = match &call.program {
            Operand::Literal(lit) => Some(literal_to_string(lit).to_uppercase()),
            _ => None, // Dynamic CALL -- skip for now
        };

        if let Some(name) = target_name {
            // Create a "Calls" edge to the target program.
            // The target program may not be in the graph yet (different source file).
            // We ensure a placeholder node exists.
            let target_id = self
                .graph
                .index()
                .lookup_one(NodeKind::Program, &name)
                .unwrap_or_else(|| {
                    self.graph
                        .add_node(Node::new(NodeKind::Program, &name))
                });

            let mut edge = Edge::new(EdgeKind::Calls);

            // Record parameter info
            let param_names: Vec<String> = call
                .using
                .iter()
                .filter_map(|p| {
                    p.operand.as_ref().map(|op| match op {
                        Operand::DataRef(dr) => dr.name.clone(),
                        Operand::Literal(lit) => literal_to_string(lit),
                        Operand::Function(f) => f.name.clone(),
                    })
                })
                .collect();

            if !param_names.is_empty() {
                edge = edge.with_property("parameters", PropValue::from(param_names));
            }

            self.graph.add_edge(prog_id, target_id, edge);
        }
    }

    fn add_para_file_access(&mut self, para_id: NodeId, file_name: &str, mode: &str) {
        let file_name_upper = file_name.to_uppercase();
        let file_id = self.ensure_file_by_name(&file_name_upper);
        self.graph.add_edge(
            para_id,
            file_id,
            Edge::new(EdgeKind::Accesses)
                .with_property("file_mode", PropValue::from(mode)),
        );
    }

    fn ensure_file_by_name(&mut self, name: &str) -> NodeId {
        if let Some(id) = self.file_ids.get(name) {
            return *id;
        }
        let id = self.graph.add_node(Node::new(NodeKind::File, name));
        self.file_ids.insert(name.to_owned(), id);
        id
    }

    fn ensure_copybook(&mut self, name: &str) -> NodeId {
        if let Some(id) = self.copybook_ids.get(name) {
            return *id;
        }
        let id = self.graph.add_node(Node::new(NodeKind::Copybook, name));
        self.copybook_ids.insert(name.to_owned(), id);
        id
    }

    fn ensure_file(&mut self, fd: &FileDescription) -> NodeId {
        if let Some(id) = self.file_ids.get(&fd.file_name) {
            return *id;
        }
        let node = Node::new(NodeKind::File, &fd.file_name)
            .with_property("organization", PropValue::from(format!("{:?}", fd.organization)))
            .with_property("access_mode", PropValue::from(format!("{:?}", fd.access_mode)));

        if let Some(assign) = &fd.assign_to {
            // Node is already built, need to create with property
            let node = node.with_property("assign_to", PropValue::from(assign.clone()));
            let id = self.graph.add_node(node);
            self.file_ids.insert(fd.file_name.clone(), id);
            return id;
        }

        let id = self.graph.add_node(node);
        self.file_ids.insert(fd.file_name.clone(), id);
        id
    }

    fn ensure_table(&mut self, name: &str) -> NodeId {
        if let Some(id) = self.table_ids.get(name) {
            return *id;
        }
        let id = self.graph.add_node(Node::new(NodeKind::Table, name));
        self.table_ids.insert(name.to_owned(), id);
        id
    }
}

impl Default for GraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn literal_to_string(lit: &Literal) -> String {
    match lit {
        Literal::Numeric(s) | Literal::Alphanumeric(s) => s.clone(),
        Literal::Figurative(f) => format!("{f:?}"),
    }
}

fn count_paragraphs(pd: &ProcedureDivision) -> usize {
    let section_paras: usize = pd.sections.iter().map(|s| s.paragraphs.len()).sum();
    section_paras + pd.paragraphs.len()
}

/// Best-effort extraction of the primary table name from raw SQL.
fn extract_table_name(raw_sql: &str) -> Option<String> {
    let upper = raw_sql.to_uppercase();
    let tokens: Vec<&str> = upper.split_whitespace().collect();

    // Look for FROM/INTO/UPDATE/INSERT INTO <table>
    for (i, token) in tokens.iter().enumerate() {
        match *token {
            "FROM" | "UPDATE" | "INTO" => {
                if let Some(table) = tokens.get(i + 1) {
                    let name = table.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                    if !name.is_empty()
                        && !["SET", "VALUES", "SELECT", "WHERE", "("].contains(&name)
                    {
                        return Some(name.to_owned());
                    }
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_table_from_select() {
        assert_eq!(
            extract_table_name("SELECT * FROM ACCTMAST WHERE STATUS = 'A'"),
            Some("ACCTMAST".to_owned())
        );
    }

    #[test]
    fn extract_table_from_update() {
        assert_eq!(
            extract_table_name("UPDATE ACCTMAST SET BAL = :WS-BAL"),
            Some("ACCTMAST".to_owned())
        );
    }

    #[test]
    fn extract_table_from_insert() {
        assert_eq!(
            extract_table_name("INSERT INTO TXNLOG VALUES (:WS-TXN)"),
            Some("TXNLOG".to_owned())
        );
    }

    #[test]
    fn extract_table_none_for_commit() {
        assert_eq!(extract_table_name("COMMIT"), None);
    }

    #[test]
    fn builder_empty_program() {
        let program = CobolProgram {
            program_id: "EMPTY".to_owned(),
            data_division: None,
            procedure_division: None,
            source_path: None,
            exec_sql_statements: Vec::new(),
        };

        let mut builder = GraphBuilder::new();
        builder.add_program(&program, &[]).unwrap();
        let graph = builder.build();

        assert_eq!(graph.node_count(), 1); // just the program node
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn builder_program_with_copybooks() {
        let program = CobolProgram {
            program_id: "PROG1".to_owned(),
            data_division: None,
            procedure_division: None,
            source_path: None,
            exec_sql_statements: Vec::new(),
        };

        let mut builder = GraphBuilder::new();
        builder
            .add_program(&program, &["CPYACCT".to_owned(), "CPYTXN".to_owned()])
            .unwrap();
        let graph = builder.build();

        // 1 program + 2 copybooks
        assert_eq!(graph.node_count(), 3);
        // 2 Uses edges
        assert_eq!(graph.edge_count(), 2);

        let prog = graph.lookup_one(NodeKind::Program, "PROG1").unwrap();
        let cbs = graph.traverse(prog, "using").unwrap();
        assert_eq!(cbs.len(), 2);
    }

    #[test]
    fn builder_deduplicates_copybooks() {
        let prog1 = CobolProgram {
            program_id: "PROG1".to_owned(),
            data_division: None,
            procedure_division: None,
            source_path: None,
            exec_sql_statements: Vec::new(),
        };
        let prog2 = CobolProgram {
            program_id: "PROG2".to_owned(),
            data_division: None,
            procedure_division: None,
            source_path: None,
            exec_sql_statements: Vec::new(),
        };

        let mut builder = GraphBuilder::new();
        builder
            .add_program(&prog1, &["CPYACCT".to_owned()])
            .unwrap();
        builder
            .add_program(&prog2, &["CPYACCT".to_owned()])
            .unwrap();
        let graph = builder.build();

        // 2 programs + 1 shared copybook
        assert_eq!(graph.node_count(), 3);
        // Both programs use the same copybook node
        let cb = graph.lookup_one(NodeKind::Copybook, "CPYACCT").unwrap();
        let users = graph.traverse(cb, "used-by").unwrap();
        assert_eq!(users.len(), 2);
    }

    #[test]
    fn builder_copybook_metadata() {
        let mut builder = GraphBuilder::new();
        builder.add_copybook_metadata("CPYMONSTER", 8000, 8, 5, 32768);
        let graph = builder.build();

        let cb = graph.lookup_one(NodeKind::Copybook, "CPYMONSTER").unwrap();
        let node = graph.node(cb).unwrap();
        assert_eq!(node.properties.get_u64("cb_field_count"), Some(8000));
        assert_eq!(node.properties.get_u64("cb_redefines_depth"), Some(8));
        assert_eq!(node.properties.get_u64("cb_occurs_count"), Some(5));
        assert_eq!(node.properties.get_u64("cb_record_length"), Some(32768));
    }

    #[test]
    fn builder_copybook_metadata_dedup() {
        let program = CobolProgram {
            program_id: "PROG1".to_owned(),
            data_division: None,
            procedure_division: None,
            source_path: None,
            exec_sql_statements: Vec::new(),
        };

        let mut builder = GraphBuilder::new();
        builder
            .add_program(&program, &["CPYACCT".to_owned()])
            .unwrap();
        // Add metadata to existing copybook -- should update same node
        builder.add_copybook_metadata("CPYACCT", 200, 2, 3, 1024);
        let graph = builder.build();

        // Still only 1 copybook node
        let cbs = graph.all_of_kind(NodeKind::Copybook);
        assert_eq!(cbs.len(), 1);

        let cb = graph.lookup_one(NodeKind::Copybook, "CPYACCT").unwrap();
        let node = graph.node(cb).unwrap();
        assert_eq!(node.properties.get_u64("cb_field_count"), Some(200));
    }

    #[test]
    fn count_paragraphs_fn() {
        use cobol_transpiler::ast::{
            Paragraph as AstParagraph, Section,
        };

        let pd = ProcedureDivision {
            using_params: Vec::new(),
            returning: None,
            sections: vec![Section {
                name: "MAIN-SECTION".to_owned(),
                paragraphs: vec![
                    AstParagraph {
                        name: "1000-INIT".to_owned(),
                        sentences: Vec::new(),
                    },
                    AstParagraph {
                        name: "2000-PROCESS".to_owned(),
                        sentences: Vec::new(),
                    },
                ],
            }],
            paragraphs: vec![AstParagraph {
                name: "9999-EXIT".to_owned(),
                sentences: Vec::new(),
            }],
        };

        assert_eq!(count_paragraphs(&pd), 3);
    }
}
