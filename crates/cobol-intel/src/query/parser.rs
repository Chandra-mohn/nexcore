use crate::error::{IntelError, IntelResult};

use super::ast::*;
use super::lexer::Token;

pub fn parse(tokens: Vec<Token>) -> IntelResult<Script> {
    let mut parser = Parser::new(tokens);
    parser.parse_script()
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn parse_script(&mut self) -> IntelResult<Script> {
        let mut statements = Vec::new();

        self.skip_newlines();

        while !self.is_at_end() {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.skip_newlines();
        }

        Ok(Script { statements })
    }

    fn parse_statement(&mut self) -> IntelResult<Statement> {
        let mut clauses = Vec::new();

        let clause = self.parse_clause()?;
        clauses.push(clause);

        // Clauses are separated by newlines within a statement
        while self.check(&Token::Newline) {
            self.skip_newlines();
            // If next token is semicolon or EOF, the statement is done
            if self.check(&Token::Semicolon) || self.is_at_end() {
                break;
            }
            let clause = self.parse_clause()?;
            clauses.push(clause);
        }

        // Expect semicolon
        if self.check(&Token::Semicolon) {
            self.advance();
        }

        Ok(Statement { clauses })
    }

    fn parse_clause(&mut self) -> IntelResult<Clause> {
        let current = self.current().clone();

        // Domain verb clause: trace, rank, similar, find-dead, deps, impact, compare, save, run
        if current.is_domain_verb() {
            return self.parse_verb_clause();
        }

        // Node type starts either a traverse or expand clause
        if current.is_node_type() {
            let node_type = self.parse_node_type()?;

            // Peek: if next is a traversal verb, it's a traverse clause
            if self.current().is_traversal_verb() {
                return self.parse_traverse_clause(node_type);
            }

            // Otherwise it's an expand clause (bare node type with optional filter)
            return self.parse_expand_clause(node_type);
        }

        Err(IntelError::ParseError {
            reason: format!(
                "expected node type or domain verb, got: {:?}",
                current
            ),
        })
    }

    fn parse_verb_clause(&mut self) -> IntelResult<Clause> {
        let verb = self.parse_domain_verb()?;

        // Optional target (identifier, each, list, or node type as target)
        let target = self.try_parse_target();

        // Modifiers
        let mut modifiers = Vec::new();
        while self.current().is_modifier() {
            let modifier = self.parse_modifier()?;
            modifiers.push(modifier);
        }

        Ok(Clause::Verb(VerbClause {
            verb,
            target,
            modifiers,
        }))
    }

    fn parse_traverse_clause(&mut self, node_type: NodeType) -> IntelResult<Clause> {
        let verb = self.parse_traversal_verb()?;

        // Target: identifier, each, or list
        let target = self.expect_target()?;

        // Optional filter
        let filter = self.try_parse_filter()?;

        Ok(Clause::Traverse(TraverseClause {
            node_type,
            verb,
            target,
            filter,
        }))
    }

    fn parse_expand_clause(&mut self, node_type: NodeType) -> IntelResult<Clause> {
        let filter = self.try_parse_filter()?;

        Ok(Clause::Expand(ExpandClause { node_type, filter }))
    }

    // --- Target parsing ---

    fn try_parse_target(&mut self) -> Option<Target> {
        match self.current() {
            Token::Identifier(_) => {
                let name = self.expect_identifier().ok()?;
                Some(Target::Identifier(name))
            }
            Token::Each => {
                self.advance();
                Some(Target::Each)
            }
            Token::LeftBracket => self.parse_list().ok().map(Target::List),
            // Node type can serve as target for verbs like `rank programs`
            t if t.is_node_type() => {
                let nt = self.parse_node_type().ok()?;
                Some(Target::Identifier(nt.as_str().to_owned()))
            }
            _ => None,
        }
    }

    fn expect_target(&mut self) -> IntelResult<Target> {
        match self.current() {
            Token::Identifier(_) => {
                let name = self.expect_identifier()?;
                Ok(Target::Identifier(name))
            }
            Token::Each => {
                self.advance();
                Ok(Target::Each)
            }
            Token::LeftBracket => {
                let list = self.parse_list()?;
                Ok(Target::List(list))
            }
            other => Err(IntelError::ParseError {
                reason: format!("expected target (identifier, 'each', or list), got: {other:?}"),
            }),
        }
    }

    // --- List parsing ---

    fn parse_list(&mut self) -> IntelResult<Vec<String>> {
        self.expect_token(&Token::LeftBracket)?;
        let mut items = Vec::new();

        if !self.check(&Token::RightBracket) {
            let name = self.expect_identifier_or_string()?;
            items.push(name);

            while self.check(&Token::Comma) {
                self.advance(); // skip comma
                let name = self.expect_identifier_or_string()?;
                items.push(name);
            }
        }

        self.expect_token(&Token::RightBracket)?;
        Ok(items)
    }

    // --- Filter parsing ---

    fn try_parse_filter(&mut self) -> IntelResult<Option<Filter>> {
        if !self.check(&Token::LeftParen) {
            return Ok(None);
        }
        self.advance(); // skip (

        let expr = self.parse_filter_expr()?;

        self.expect_token(&Token::RightParen)?;

        Ok(Some(Filter {
            predicates: vec![expr],
        }))
    }

    fn parse_filter_expr(&mut self) -> IntelResult<FilterExpr> {
        let mut left = self.parse_filter_atom()?;

        loop {
            if self.check(&Token::And) {
                self.advance();
                let right = self.parse_filter_atom()?;
                left = FilterExpr::And(Box::new(left), Box::new(right));
            } else if self.check(&Token::Or) {
                self.advance();
                let right = self.parse_filter_atom()?;
                left = FilterExpr::Or(Box::new(left), Box::new(right));
            } else if self.check(&Token::Comma) {
                // Comma in filter = AND
                self.advance();
                let right = self.parse_filter_atom()?;
                left = FilterExpr::And(Box::new(left), Box::new(right));
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_filter_atom(&mut self) -> IntelResult<FilterExpr> {
        if self.check(&Token::Not) {
            self.advance();
            let inner = self.parse_filter_atom()?;
            return Ok(FilterExpr::Not(Box::new(inner)));
        }

        let field = self.expect_identifier()?;
        let op = self.parse_compare_op()?;
        let value = self.parse_value()?;

        Ok(FilterExpr::Predicate(Predicate { field, op, value }))
    }

    fn parse_compare_op(&mut self) -> IntelResult<CompareOp> {
        let op = match self.current() {
            Token::Eq => CompareOp::Eq,
            Token::NotEq => CompareOp::NotEq,
            Token::Gt => CompareOp::Gt,
            Token::Lt => CompareOp::Lt,
            Token::Gte => CompareOp::Gte,
            Token::Lte => CompareOp::Lte,
            Token::Tilde => CompareOp::Glob,
            Token::DoubleTilde => CompareOp::Regex,
            Token::In => CompareOp::In,
            Token::Has => CompareOp::Has,
            other => {
                return Err(IntelError::ParseError {
                    reason: format!("expected comparison operator, got: {other:?}"),
                });
            }
        };
        self.advance();
        Ok(op)
    }

    fn parse_value(&mut self) -> IntelResult<Value> {
        match self.current().clone() {
            Token::StringLit(s) => {
                self.advance();
                Ok(Value::String(s))
            }
            Token::NumberLit(n) => {
                self.advance();
                Ok(Value::Number(n))
            }
            Token::LeftBracket => {
                let items = self.parse_list()?;
                Ok(Value::List(items))
            }
            other => Err(IntelError::ParseError {
                reason: format!("expected value (string, number, or list), got: {other:?}"),
            }),
        }
    }

    // --- Modifier parsing ---

    fn parse_modifier(&mut self) -> IntelResult<Modifier> {
        let keyword = match self.current() {
            Token::By => ModifierKeyword::By,
            Token::Top => ModifierKeyword::Top,
            Token::In => ModifierKeyword::In,
            Token::With => ModifierKeyword::With,
            Token::Depth => ModifierKeyword::Depth,
            Token::Level => ModifierKeyword::Level,
            Token::Order => ModifierKeyword::Order,
            Token::Scope => ModifierKeyword::Scope,
            Token::Threshold => ModifierKeyword::Threshold,
            Token::As => ModifierKeyword::As,
            other => {
                return Err(IntelError::ParseError {
                    reason: format!("expected modifier keyword, got: {other:?}"),
                });
            }
        };
        self.advance();

        // `as` is a marker-only modifier for `save ... as` -- no value needed.
        // The query body follows on subsequent lines as clauses.
        if keyword == ModifierKeyword::As {
            return Ok(Modifier {
                keyword,
                value: ModifierValue::Identifier(String::new()),
            });
        }

        // Modifier value: identifier, number, or node type
        let value = match self.current().clone() {
            Token::NumberLit(n) => {
                self.advance();
                ModifierValue::Number(n)
            }
            Token::Identifier(s) => {
                self.advance();
                ModifierValue::Identifier(s)
            }
            t if t.is_node_type() => {
                let nt = self.parse_node_type()?;
                ModifierValue::NodeType(nt)
            }
            other => {
                return Err(IntelError::ParseError {
                    reason: format!("expected modifier value, got: {other:?}"),
                });
            }
        };

        Ok(Modifier { keyword, value })
    }

    // --- Enum parsing helpers ---

    fn parse_node_type(&mut self) -> IntelResult<NodeType> {
        let nt = match self.current() {
            Token::Programs => NodeType::Programs,
            Token::Paragraphs => NodeType::Paragraphs,
            Token::Fields => NodeType::Fields,
            Token::Copybooks => NodeType::Copybooks,
            Token::Files => NodeType::Files,
            Token::Tables => NodeType::Tables,
            Token::Rules => NodeType::Rules,
            other => {
                return Err(IntelError::ParseError {
                    reason: format!("expected node type, got: {other:?}"),
                });
            }
        };
        self.advance();
        Ok(nt)
    }

    fn parse_domain_verb(&mut self) -> IntelResult<DomainVerb> {
        let verb = match self.current() {
            Token::Trace => DomainVerb::Trace,
            Token::Rank => DomainVerb::Rank,
            Token::Similar => DomainVerb::Similar,
            Token::FindDead => DomainVerb::FindDead,
            Token::DiscoverProcesses => DomainVerb::DiscoverProcesses,
            Token::EstimateCost => DomainVerb::EstimateCost,
            Token::Deps => DomainVerb::Deps,
            Token::Impact => DomainVerb::Impact,
            Token::Compare => DomainVerb::Compare,
            Token::Save => DomainVerb::Save,
            Token::Run => DomainVerb::Run,
            other => {
                return Err(IntelError::ParseError {
                    reason: format!("expected domain verb, got: {other:?}"),
                });
            }
        };
        self.advance();
        Ok(verb)
    }

    fn parse_traversal_verb(&mut self) -> IntelResult<TraversalVerb> {
        let verb = match self.current() {
            Token::Calling => TraversalVerb::Calling,
            Token::CalledBy => TraversalVerb::CalledBy,
            Token::Performing => TraversalVerb::Performing,
            Token::PerformedBy => TraversalVerb::PerformedBy,
            Token::Reading => TraversalVerb::Reading,
            Token::ReadBy => TraversalVerb::ReadBy,
            Token::Writing => TraversalVerb::Writing,
            Token::WrittenBy => TraversalVerb::WrittenBy,
            Token::Using => TraversalVerb::Using,
            Token::UsedBy => TraversalVerb::UsedBy,
            Token::Accessing => TraversalVerb::Accessing,
            Token::AccessedBy => TraversalVerb::AccessedBy,
            Token::Containing => TraversalVerb::Containing,
            Token::Within => TraversalVerb::Within,
            other => {
                return Err(IntelError::ParseError {
                    reason: format!("expected traversal verb, got: {other:?}"),
                });
            }
        };
        self.advance();
        Ok(verb)
    }

    // --- Utility methods ---

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn check(&self, expected: &Token) -> bool {
        std::mem::discriminant(self.current()) == std::mem::discriminant(expected)
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current(), Token::Eof)
    }

    fn skip_newlines(&mut self) {
        while self.check(&Token::Newline) {
            self.advance();
        }
    }

    fn expect_token(&mut self, expected: &Token) -> IntelResult<()> {
        if self.check(expected) {
            self.advance();
            Ok(())
        } else {
            Err(IntelError::ParseError {
                reason: format!("expected {expected:?}, got: {:?}", self.current()),
            })
        }
    }

    fn expect_identifier(&mut self) -> IntelResult<String> {
        match self.current().clone() {
            Token::Identifier(s) => {
                self.advance();
                Ok(s)
            }
            other => Err(IntelError::ParseError {
                reason: format!("expected identifier, got: {other:?}"),
            }),
        }
    }

    fn expect_identifier_or_string(&mut self) -> IntelResult<String> {
        match self.current().clone() {
            Token::Identifier(s) | Token::StringLit(s) => {
                self.advance();
                Ok(s)
            }
            other => Err(IntelError::ParseError {
                reason: format!("expected identifier or string, got: {other:?}"),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::lexer::Lexer;

    fn parse_nxq(input: &str) -> Script {
        let tokens = Lexer::new(input).tokenize().unwrap();
        parse(tokens).unwrap()
    }

    fn parse_nxq_err(input: &str) -> String {
        let tokens = Lexer::new(input).tokenize().unwrap();
        parse(tokens).unwrap_err().to_string()
    }

    fn first_clause(input: &str) -> Clause {
        let script = parse_nxq(input);
        script.statements[0].clauses[0].clone()
    }

    // === Pattern 1: Process Trace ===

    #[test]
    fn parse_trace_depth_full() {
        let script = parse_nxq("trace CLRG0100 depth full;");
        assert_eq!(script.statements.len(), 1);
        let clause = &script.statements[0].clauses[0];
        match clause {
            Clause::Verb(v) => {
                assert_eq!(v.verb, DomainVerb::Trace);
                assert_eq!(v.target, Some(Target::Identifier("CLRG0100".to_owned())));
                assert_eq!(v.modifiers.len(), 1);
                assert_eq!(v.modifiers[0].keyword, ModifierKeyword::Depth);
            }
            _ => panic!("expected verb clause"),
        }
    }

    // === Pattern 2: Data Carving ===

    #[test]
    fn parse_data_carving() {
        let script = parse_nxq(
            "programs writing [WS-ACCT-NBR, WS-ACCT-BAL]\nrank by loc;",
        );
        assert_eq!(script.statements.len(), 1);
        assert_eq!(script.statements[0].clauses.len(), 2);

        match &script.statements[0].clauses[0] {
            Clause::Traverse(t) => {
                assert_eq!(t.node_type, NodeType::Programs);
                assert_eq!(t.verb, TraversalVerb::Writing);
                assert_eq!(
                    t.target,
                    Target::List(vec!["WS-ACCT-NBR".to_owned(), "WS-ACCT-BAL".to_owned()])
                );
            }
            _ => panic!("expected traverse clause"),
        }

        match &script.statements[0].clauses[1] {
            Clause::Verb(v) => {
                assert_eq!(v.verb, DomainVerb::Rank);
                assert_eq!(v.modifiers[0].keyword, ModifierKeyword::By);
            }
            _ => panic!("expected verb clause"),
        }
    }

    // === Pattern 3: Impact Analysis ===

    #[test]
    fn parse_impact_analysis() {
        let script = parse_nxq(
            "programs using CPYCLRG\nprograms calling each\nrank by risk top 20;",
        );
        assert_eq!(script.statements[0].clauses.len(), 3);
    }

    // === Pattern 4: Similarity Search ===

    #[test]
    fn parse_similarity() {
        let script = parse_nxq("similar CLRG0100 by structure threshold 0.7;");
        match &script.statements[0].clauses[0] {
            Clause::Verb(v) => {
                assert_eq!(v.verb, DomainVerb::Similar);
                assert_eq!(v.modifiers.len(), 2);
                assert_eq!(v.modifiers[0].keyword, ModifierKeyword::By);
                assert_eq!(v.modifiers[1].keyword, ModifierKeyword::Threshold);
                assert_eq!(v.modifiers[1].value, ModifierValue::Number(0.7));
            }
            _ => panic!("expected verb clause"),
        }
    }

    // === Pattern 5: Dead Code Detection ===

    #[test]
    fn parse_find_dead() {
        let script = parse_nxq("find-dead level paragraph in CLRG0100;");
        match &script.statements[0].clauses[0] {
            Clause::Verb(v) => {
                assert_eq!(v.verb, DomainVerb::FindDead);
                assert_eq!(v.modifiers.len(), 2);
                assert_eq!(v.modifiers[0].keyword, ModifierKeyword::Level);
                assert_eq!(v.modifiers[1].keyword, ModifierKeyword::In);
            }
            _ => panic!("expected verb clause"),
        }
    }

    // === Pattern 6: Dependency Chain ===

    #[test]
    fn parse_deps() {
        let script = parse_nxq("deps CLRG0100 order topological;");
        match &script.statements[0].clauses[0] {
            Clause::Verb(v) => {
                assert_eq!(v.verb, DomainVerb::Deps);
                assert_eq!(v.target, Some(Target::Identifier("CLRG0100".to_owned())));
                assert_eq!(v.modifiers[0].keyword, ModifierKeyword::Order);
            }
            _ => panic!("expected verb clause"),
        }
    }

    // === Pattern 7: Complexity Hotspots ===

    #[test]
    fn parse_rank_programs() {
        let script = parse_nxq("rank programs by complexity top 20;");
        match &script.statements[0].clauses[0] {
            Clause::Verb(v) => {
                assert_eq!(v.verb, DomainVerb::Rank);
                assert_eq!(
                    v.target,
                    Some(Target::Identifier("programs".to_owned()))
                );
                assert_eq!(v.modifiers.len(), 2);
            }
            _ => panic!("expected verb clause"),
        }
    }

    // === Pattern 8: Rule Comparison ===

    #[test]
    fn parse_compare_rules() {
        let script = parse_nxq("compare rules in CLRG0100 with CLRG0100-V2;");
        match &script.statements[0].clauses[0] {
            Clause::Verb(v) => {
                assert_eq!(v.verb, DomainVerb::Compare);
                assert_eq!(v.modifiers.len(), 2);
                assert_eq!(v.modifiers[0].keyword, ModifierKeyword::In);
                assert_eq!(v.modifiers[1].keyword, ModifierKeyword::With);
            }
            _ => panic!("expected verb clause"),
        }
    }

    // === Composed queries ===

    #[test]
    fn parse_composed_trace_find_dead() {
        let script = parse_nxq("trace CLRG0100\nfind-dead level paragraph;");
        assert_eq!(script.statements[0].clauses.len(), 2);
        match &script.statements[0].clauses[0] {
            Clause::Verb(v) => assert_eq!(v.verb, DomainVerb::Trace),
            _ => panic!("expected verb"),
        }
        match &script.statements[0].clauses[1] {
            Clause::Verb(v) => assert_eq!(v.verb, DomainVerb::FindDead),
            _ => panic!("expected verb"),
        }
    }

    #[test]
    fn parse_composed_with_filter() {
        let script = parse_nxq(
            "copybooks used-by CLRG0100\nprograms using each(complexity > 4.0);",
        );
        assert_eq!(script.statements[0].clauses.len(), 2);

        match &script.statements[0].clauses[1] {
            Clause::Traverse(t) => {
                assert_eq!(t.target, Target::Each);
                assert!(t.filter.is_some());
                let filter = t.filter.as_ref().unwrap();
                match &filter.predicates[0] {
                    FilterExpr::Predicate(p) => {
                        assert_eq!(p.field, "complexity");
                        assert_eq!(p.op, CompareOp::Gt);
                        assert_eq!(p.value, Value::Number(4.0));
                    }
                    _ => panic!("expected predicate"),
                }
            }
            _ => panic!("expected traverse"),
        }
    }

    #[test]
    fn parse_composed_access_then_rules() {
        let script = parse_nxq(
            "programs accessing ACCTMAST(mode = 'write')\nrules;",
        );
        assert_eq!(script.statements[0].clauses.len(), 2);

        match &script.statements[0].clauses[0] {
            Clause::Traverse(t) => {
                assert_eq!(t.node_type, NodeType::Programs);
                assert!(t.filter.is_some());
            }
            _ => panic!("expected traverse"),
        }
        match &script.statements[0].clauses[1] {
            Clause::Expand(e) => {
                assert_eq!(e.node_type, NodeType::Rules);
            }
            _ => panic!("expected expand"),
        }
    }

    #[test]
    fn parse_expand_fields() {
        let clause = first_clause("fields;");
        match clause {
            Clause::Expand(e) => assert_eq!(e.node_type, NodeType::Fields),
            _ => panic!("expected expand"),
        }
    }

    // === Filters ===

    #[test]
    fn parse_filter_and() {
        let script = parse_nxq("programs calling CLRG0100(complexity > 3 and type = 'batch');");
        match &script.statements[0].clauses[0] {
            Clause::Traverse(t) => {
                let filter = t.filter.as_ref().unwrap();
                match &filter.predicates[0] {
                    FilterExpr::And(_, _) => {} // ok
                    other => panic!("expected And, got: {other:?}"),
                }
            }
            _ => panic!("expected traverse"),
        }
    }

    #[test]
    fn parse_filter_or() {
        let script = parse_nxq("programs calling CLRG0100(type = 'batch' or type = 'online');");
        match &script.statements[0].clauses[0] {
            Clause::Traverse(t) => {
                let filter = t.filter.as_ref().unwrap();
                match &filter.predicates[0] {
                    FilterExpr::Or(_, _) => {}
                    other => panic!("expected Or, got: {other:?}"),
                }
            }
            _ => panic!("expected traverse"),
        }
    }

    #[test]
    fn parse_filter_not() {
        let script = parse_nxq("programs calling CLRG0100(not type = 'dead');");
        match &script.statements[0].clauses[0] {
            Clause::Traverse(t) => {
                let filter = t.filter.as_ref().unwrap();
                match &filter.predicates[0] {
                    FilterExpr::Not(_) => {}
                    other => panic!("expected Not, got: {other:?}"),
                }
            }
            _ => panic!("expected traverse"),
        }
    }

    #[test]
    fn parse_filter_in_list() {
        let script = parse_nxq("programs calling CLRG0100(type in ['batch', 'online']);");
        match &script.statements[0].clauses[0] {
            Clause::Traverse(t) => {
                let filter = t.filter.as_ref().unwrap();
                match &filter.predicates[0] {
                    FilterExpr::Predicate(p) => {
                        assert_eq!(p.op, CompareOp::In);
                    }
                    _ => panic!("expected predicate"),
                }
            }
            _ => panic!("expected traverse"),
        }
    }

    #[test]
    fn parse_filter_glob() {
        let script = parse_nxq("programs calling CLRG0100(name ~ 'CLRG*');");
        match &script.statements[0].clauses[0] {
            Clause::Traverse(t) => {
                let filter = t.filter.as_ref().unwrap();
                match &filter.predicates[0] {
                    FilterExpr::Predicate(p) => {
                        assert_eq!(p.op, CompareOp::Glob);
                        assert_eq!(p.value, Value::String("CLRG*".to_owned()));
                    }
                    _ => panic!("expected predicate"),
                }
            }
            _ => panic!("expected traverse"),
        }
    }

    // === Save / Run ===

    #[test]
    fn parse_save_query() {
        let script = parse_nxq(
            "save clearing-blast as\n  programs using CPYCLRG\n  programs calling each\n  rank by risk;",
        );
        assert_eq!(script.statements[0].clauses.len(), 4);
        match &script.statements[0].clauses[0] {
            Clause::Verb(v) => {
                assert_eq!(v.verb, DomainVerb::Save);
                assert_eq!(
                    v.target,
                    Some(Target::Identifier("clearing-blast".to_owned()))
                );
                assert_eq!(v.modifiers.len(), 1);
                assert_eq!(v.modifiers[0].keyword, ModifierKeyword::As);
            }
            _ => panic!("expected verb"),
        }
    }

    #[test]
    fn parse_run_query() {
        let script = parse_nxq("run clearing-blast;");
        match &script.statements[0].clauses[0] {
            Clause::Verb(v) => {
                assert_eq!(v.verb, DomainVerb::Run);
                assert_eq!(
                    v.target,
                    Some(Target::Identifier("clearing-blast".to_owned()))
                );
            }
            _ => panic!("expected verb"),
        }
    }

    // === Multiple statements ===

    #[test]
    fn parse_multiple_statements() {
        let script = parse_nxq(
            "rank programs by complexity top 10;\n\nfind-dead level program scope all;",
        );
        assert_eq!(script.statements.len(), 2);
    }

    // === Edge cases ===

    #[test]
    fn parse_with_comments() {
        let script = parse_nxq(
            "-- this is a comment\ntrace CLRG0100 depth full;\n-- another comment",
        );
        assert_eq!(script.statements.len(), 1);
    }

    #[test]
    fn parse_continuation_dots() {
        let script = parse_nxq(
            "programs calling CLRG0100\n  .. rank by complexity;",
        );
        assert_eq!(script.statements[0].clauses.len(), 2);
    }

    // === Error cases ===

    #[test]
    fn error_unexpected_token() {
        let err = parse_nxq_err("42;");
        assert!(err.contains("expected"), "got: {err}");
    }

    #[test]
    fn error_missing_semicolon_still_parses() {
        // Graceful: missing semicolon at end should still parse
        let script = parse_nxq("trace CLRG0100");
        assert_eq!(script.statements.len(), 1);
    }

    // === The "real-world session" from the spec ===

    #[test]
    fn parse_real_world_session() {
        let input = r#"
-- Morning: what are we dealing with?
rank programs by complexity top 20;

-- What does CLRG0100 do?
trace CLRG0100 depth full;

-- What copybooks does the clearing cluster share?
programs calling [CLRG0100, CLRG0200, CLRG0300]
copybooks;

-- Any dead code?
find-dead level paragraph in CLRG0100;

-- Save what we learned
save clearing-scope as
  programs calling [CLRG0100, CLRG0200, CLRG0300]
  programs calling each;

-- Pick up later
run clearing-scope;
"#;
        let script = parse_nxq(input);
        assert_eq!(script.statements.len(), 6);
    }
}
