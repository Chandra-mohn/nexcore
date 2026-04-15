//! Typed DSL AST: grammar-valid by construction.
//!
//! These types mirror the 4 Nexflow DSL grammars (SchemaDSL, TransformDSL,
//! RulesDSL, ProcDSL). If you can construct a value, it serializes to
//! grammar-valid DSL text. No confidence scores needed.
//!
//! When emission fails (can't extract enough data to build a valid AST node),
//! a `DslDiagnostic` is produced instead -- a structured debug record that
//! captures what was seen and why generation failed.

use std::fmt;
use std::fmt::Write as _;

use serde::Serialize;

// ============================================================================
// Common Types
// ============================================================================

/// A valid DSL identifier: [a-z_][a-z0-9_]*
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Ident(String);

impl Ident {
    /// Create a validated identifier. Sanitizes input to be grammar-valid.
    pub fn new(s: &str) -> Self {
        let sanitized = s.to_lowercase().replace(['-', ' '], "_");
        let sanitized = if sanitized.starts_with(|c: char| c.is_ascii_digit()) {
            format!("_{sanitized}")
        } else if sanitized.is_empty() {
            "unnamed".to_string()
        } else {
            sanitized
        };
        // Strip any non-alphanumeric/underscore chars
        let clean: String = sanitized
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect();
        Self(if clean.is_empty() { "unnamed".to_string() } else { clean })
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Import path: ../schema/foo.schema
#[derive(Debug, Clone, Serialize)]
pub struct ImportPath {
    pub path: String,
}

impl ImportPath {
    pub fn schema(name: &str) -> Self {
        Self { path: format!("../schema/{name}.schema") }
    }
    pub fn transform(name: &str) -> Self {
        Self { path: format!("../transform/{name}.xform") }
    }
    pub fn rules(name: &str) -> Self {
        Self { path: format!("../rules/{name}.rules") }
    }
}

/// A comment line in the DSL output.
#[derive(Debug, Clone, Serialize)]
pub struct Comment(pub String);

/// DSL literal value.
#[derive(Debug, Clone, Serialize)]
pub enum Literal {
    Str(String),
    Int(i64),
    Decimal(String), // Preserve precision as string
    Bool(bool),
    Null,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Str(s) => write!(f, "\"{s}\""),
            Self::Int(n) => write!(f, "{n}"),
            Self::Decimal(d) => write!(f, "{d}"),
            Self::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Self::Null => write!(f, "null"),
        }
    }
}

/// DSL expression (shared across Rules and Transform).
#[derive(Debug, Clone, Serialize)]
pub enum Expr {
    Field(Ident),
    Lit(Literal),
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Call(Ident, Vec<Expr>),
    FieldPath(Vec<Ident>),
    /// Raw expression string (fallback for patterns we can't fully type)
    Raw(String),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Field(id) => write!(f, "{id}"),
            Self::Lit(lit) => write!(f, "{lit}"),
            Self::Binary(l, op, r) => write!(f, "{l} {op} {r}"),
            Self::Unary(op, e) => write!(f, "{op} ({e})"),
            Self::Call(name, args) => {
                write!(f, "{name}(")?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{arg}")?;
                }
                write!(f, ")")
            }
            Self::FieldPath(parts) => {
                for (i, p) in parts.iter().enumerate() {
                    if i > 0 { write!(f, ".")?; }
                    write!(f, "{p}")?;
                }
                Ok(())
            }
            Self::Raw(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    Gt, Lt, Ge, Le, Eq, Ne,
    And, Or,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Add => "+", Self::Sub => "-", Self::Mul => "*",
            Self::Div => "/", Self::Mod => "%",
            Self::Gt => ">", Self::Lt => "<", Self::Ge => ">=",
            Self::Le => "<=", Self::Eq => "==", Self::Ne => "!=",
            Self::And => "and", Self::Or => "or",
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum UnaryOp { Not, Neg }

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Not => "not", Self::Neg => "-",
        })
    }
}

// ============================================================================
// SchemaDSL AST
// ============================================================================

/// A complete .schema file.
#[derive(Debug, Clone, Serialize)]
pub struct SchemaFile {
    pub comments: Vec<Comment>,
    pub imports: Vec<ImportPath>,
    pub schemas: Vec<SchemaDef>,
}

/// One `schema ... end` block.
#[derive(Debug, Clone, Serialize)]
pub struct SchemaDef {
    pub name: Ident,
    pub pattern: Option<MutationPattern>,
    pub identity: Option<Vec<FieldDecl>>,
    pub fields: Vec<FieldDecl>,
    pub nested_objects: Vec<NestedObjectDef>,
    pub constraints: Vec<SchemaConstraint>,
}

/// A nested object block: `name object ... end` or `name list<object> ... end`
#[derive(Debug, Clone, Serialize)]
pub struct NestedObjectDef {
    pub name: Ident,
    /// If true, renders as `name list<object> ... end` (OCCURS).
    /// If false, renders as `name object ... end` (sub-group).
    pub is_list: bool,
    pub fields: Vec<FieldDecl>,
    pub nested: Vec<NestedObjectDef>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum MutationPattern {
    MasterData,
    EventLog,
    StateMachine,
    ReferenceData,
}

impl fmt::Display for MutationPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::MasterData => "master_data",
            Self::EventLog => "event_log",
            Self::StateMachine => "state_machine",
            Self::ReferenceData => "reference_data",
        })
    }
}

/// A field declaration in a schema.
#[derive(Debug, Clone, Serialize)]
pub struct FieldDecl {
    pub name: Ident,
    pub field_type: FieldType,
    pub required: bool,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum FieldType {
    String(Option<usize>),
    Char(usize),
    Integer(Option<usize>),
    Decimal(usize, usize),
    Boolean,
    Date,
    Timestamp,
    Float,
    List(Box<FieldType>),
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(None) => write!(f, "string"),
            Self::String(Some(n)) => write!(f, "string({n})"),
            Self::Char(n) => write!(f, "char({n})"),
            Self::Integer(None) => write!(f, "integer"),
            Self::Integer(Some(p)) => write!(f, "integer({p})"),
            Self::Decimal(p, s) => write!(f, "decimal({p}, {s})"),
            Self::Boolean => write!(f, "boolean"),
            Self::Date => write!(f, "date"),
            Self::Timestamp => write!(f, "timestamp"),
            Self::Float => write!(f, "float"),
            Self::List(inner) => write!(f, "list({inner})"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum SchemaConstraint {
    Enum(Ident, Vec<String>),
}

// ============================================================================
// RulesDSL AST
// ============================================================================

/// A complete .rules file.
#[derive(Debug, Clone, Serialize)]
pub struct RulesFile {
    pub comments: Vec<Comment>,
    pub imports: Vec<ImportPath>,
    pub items: Vec<RuleItem>,
}

#[derive(Debug, Clone, Serialize)]
pub enum RuleItem {
    DecisionTable(DecisionTable),
    ProceduralRule(ProceduralRule),
}

/// `decision_table ... end` block.
#[derive(Debug, Clone, Serialize)]
pub struct DecisionTable {
    pub name: Ident,
    pub comment: Option<String>,
    pub hit_policy: HitPolicy,
    pub description: Option<String>,
    pub version: Option<String>,
    pub given: Vec<GivenParam>,
    pub decide: DecideMatrix,
    pub return_params: Vec<ReturnParam>,
    pub post_calculate: Option<Vec<RuleStmt>>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum HitPolicy { FirstMatch, SingleHit, MultiHit, CollectAll }

impl fmt::Display for HitPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::FirstMatch => "first_match",
            Self::SingleHit => "single_hit",
            Self::MultiHit => "multi_hit",
            Self::CollectAll => "collect_all",
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GivenParam {
    pub name: Ident,
    pub param_type: RulesParamType,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum RulesParamType {
    Text, Number, Integer, Boolean, Date, Timestamp, Money, Percentage,
}

impl fmt::Display for RulesParamType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Text => "text", Self::Number => "number",
            Self::Integer => "integer", Self::Boolean => "boolean",
            Self::Date => "date", Self::Timestamp => "timestamp",
            Self::Money => "money", Self::Percentage => "percentage",
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DecideMatrix {
    pub condition_col: String,
    pub action_cols: Vec<Ident>,
    pub rows: Vec<DecideRow>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DecideRow {
    pub condition: String,
    pub actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReturnParam {
    pub name: Ident,
    pub param_type: RulesParamType,
}

/// `rule ... : ... end` block.
#[derive(Debug, Clone, Serialize)]
pub struct ProceduralRule {
    pub name: Ident,
    pub comment: Option<String>,
    pub description: Option<String>,
    pub body: Vec<RuleStmt>,
}

#[derive(Debug, Clone, Serialize)]
pub enum RuleStmt {
    If(IfStmt),
    Set(Ident, Expr),
    Let(Ident, Expr),
    Return,
}

#[derive(Debug, Clone, Serialize)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_block: Vec<RuleStmt>,
    pub elseif_blocks: Vec<(Expr, Vec<RuleStmt>)>,
    pub else_block: Option<Vec<RuleStmt>>,
}

// ============================================================================
// TransformDSL AST
// ============================================================================

/// A complete .xform file.
#[derive(Debug, Clone, Serialize)]
pub struct TransformFile {
    pub comments: Vec<Comment>,
    pub imports: Vec<ImportPath>,
    pub items: Vec<TransformItem>,
}

#[derive(Debug, Clone, Serialize)]
pub enum TransformItem {
    Transform(TransformDef),
    TransformBlock(TransformBlockDef),
}

/// `transform ... end` (single field).
#[derive(Debug, Clone, Serialize)]
pub struct TransformDef {
    pub name: Ident,
    pub comment: Option<String>,
    pub metadata: Option<TransformMetadata>,
    pub pure: bool,
    pub cache: Option<CacheDecl>,
    pub input: IoSpec,
    pub output: IoSpec,
    pub apply: Vec<ApplyStmt>,
    pub validate_input: Option<Vec<ValidationRule>>,
    pub validate_output: Option<Vec<ValidationRule>>,
    pub on_error: Option<Vec<ErrorStatement>>,
}

/// `transform_block ... end` (multi-field or compose).
#[derive(Debug, Clone, Serialize)]
pub struct TransformBlockDef {
    pub name: Ident,
    pub comment: Option<String>,
    pub metadata: Option<TransformMetadata>,
    pub use_decls: Option<Vec<Ident>>,
    pub input: IoSpec,
    pub output: IoSpec,
    pub body: TransformBlockBody,
    pub validate_input: Option<Vec<ValidationRule>>,
    pub validate_output: Option<Vec<ValidationRule>>,
    pub on_error: Option<Vec<ErrorStatement>>,
}

// ============================================================================
// Validation + Error Handling Types
// ============================================================================

/// A validation rule for validate_input / validate_output blocks.
#[derive(Debug, Clone, Serialize)]
pub enum ValidationRule {
    /// `require expr else "message"`
    Require(Expr, ValidationMessage),
    /// `expr : "message"`
    Simple(Expr, ValidationMessage),
}

/// Message attached to a validation rule.
#[derive(Debug, Clone, Serialize)]
pub struct ValidationMessage {
    pub text: String,
    pub code: Option<String>,
    pub severity: Option<Severity>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
        })
    }
}

/// A statement inside an `on_error ... end` block.
#[derive(Debug, Clone, Serialize)]
pub enum ErrorStatement {
    /// `action : reject|skip|use_default|raise`
    Action(ErrorAction),
    /// `log_error("message")`
    LogError(String),
    /// `error_code : "CODE"`
    ErrorCode(String),
    /// `emit with defaults|partial`
    Emit(EmitMode),
    /// `reject with "message"` or `reject with code "CODE"`
    Reject(String),
    /// `default : expr`
    Default(Expr),
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum ErrorAction {
    Reject,
    Skip,
    UseDefault,
    Raise,
}

impl fmt::Display for ErrorAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Reject => "reject",
            Self::Skip => "skip",
            Self::UseDefault => "use_default",
            Self::Raise => "raise",
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum EmitMode {
    Defaults,
    Partial,
}

// ============================================================================
// Metadata, Use, Cache Types
// ============================================================================

/// Transform metadata (version, description).
#[derive(Debug, Clone, Serialize)]
pub struct TransformMetadata {
    pub version: Option<String>,
    pub description: Option<String>,
}

/// Cache declaration for transforms.
#[derive(Debug, Clone, Serialize)]
pub struct CacheDecl {
    /// Cache TTL as a raw duration string (e.g., "5 minutes", "3600s").
    pub ttl: Option<String>,
    /// Cache key fields.
    pub key: Option<Vec<Ident>>,
}

#[derive(Debug, Clone, Serialize)]
pub enum IoSpec {
    Single(Ident, FieldType),
    Multi(Vec<IoField>),
}

#[derive(Debug, Clone, Serialize)]
pub struct IoField {
    pub name: Ident,
    pub field_type: FieldType,
}

#[derive(Debug, Clone, Serialize)]
pub enum ApplyStmt {
    Assign(Ident, Expr),
    Let(Ident, Expr),
}

#[derive(Debug, Clone, Serialize)]
pub enum TransformBlockBody {
    Mappings(Vec<MappingEntry>),
    Compose(ComposeBlock),
}

#[derive(Debug, Clone, Serialize)]
pub struct MappingEntry {
    pub target: Ident,
    pub source: Expr,
}

#[derive(Debug, Clone, Serialize)]
pub struct ComposeBlock {
    pub compose_type: ComposeType,
    pub refs: Vec<ComposeRef>,
}

/// A reference inside a compose block, optionally conditional.
#[derive(Debug, Clone, Serialize)]
pub enum ComposeRef {
    /// Simple transform reference: `transform_name`
    Simple(Ident),
    /// Conditional reference: `when expr : transform_name`
    When(Expr, Ident),
    /// Default fallback: `otherwise : transform_name`
    Otherwise(Ident),
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum ComposeType { Sequential, Parallel, Conditional }

impl fmt::Display for ComposeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Sequential => "sequential",
            Self::Parallel => "parallel",
            Self::Conditional => "conditional",
        })
    }
}

// ============================================================================
// ProcDSL AST
// ============================================================================

/// A complete .proc file.
#[derive(Debug, Clone, Serialize)]
pub struct ProcessFile {
    pub comments: Vec<Comment>,
    pub imports: Vec<ImportPath>,
    pub processes: Vec<ProcessDef>,
}

/// `process ... end` block.
#[derive(Debug, Clone, Serialize)]
pub struct ProcessDef {
    pub name: Ident,
    pub mode: Option<ProcessMode>,
    pub execution: Option<ExecutionBlock>,
    pub body: Vec<ProcessStmt>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum ProcessMode { Stream, Batch }

impl fmt::Display for ProcessMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Stream => "stream",
            Self::Batch => "batch",
        })
    }
}

/// Execution block: parallelism and partitioning hints.
#[derive(Debug, Clone, Serialize)]
pub struct ExecutionBlock {
    pub parallelism: Option<u32>,
    pub partition_by: Option<Vec<Ident>>,
}

#[derive(Debug, Clone, Serialize)]
pub enum ProcessStmt {
    Comment(String),
    Receive(ReceiveBlock),
    TransformUsing { input: Ident, using: Ident, output: Ident },
    EvaluateUsing(Ident),
    Emit(EmitBlock),
    /// Conditional routing: `route using rule_name ...`
    Route(RouteBlock),
    /// Parallel fan-out: `parallel ... branch ... end`
    Parallel(ParallelBlock),
    /// Loop: `loop ... end` (from PERFORM UNTIL)
    Loop(LoopBlock),
}

#[derive(Debug, Clone, Serialize)]
pub struct ReceiveBlock {
    pub name: Ident,
    pub schema: Option<Ident>,
    pub connector: Option<ConnectorSpec>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmitBlock {
    pub target: Ident,
    pub schema: Option<Ident>,
    pub connector: Option<ConnectorSpec>,
}

/// Connector specification for receive/emit.
#[derive(Debug, Clone, Serialize)]
pub struct ConnectorSpec {
    pub connector_type: ConnectorType,
    pub config: Option<String>,
}

/// Connector type (source/target agnostic).
#[derive(Debug, Clone, Copy, Serialize)]
pub enum ConnectorType {
    File,
    Csv,
    Parquet,
    Db,
    Kafka,
    StateStore,
}

impl fmt::Display for ConnectorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::File => "file",
            Self::Csv => "csv",
            Self::Parquet => "parquet",
            Self::Db => "db",
            Self::Kafka => "kafka",
            Self::StateStore => "state_store",
        })
    }
}

/// Route block: conditional routing to different targets.
#[derive(Debug, Clone, Serialize)]
pub struct RouteBlock {
    pub using: Ident,
    pub destinations: Vec<(Option<String>, Ident)>,
    pub otherwise: Option<Ident>,
}

/// Parallel fan-out block.
#[derive(Debug, Clone, Serialize)]
pub struct ParallelBlock {
    pub branches: Vec<(Ident, Vec<ProcessStmt>)>,
}

/// Loop block (from PERFORM UNTIL).
#[derive(Debug, Clone, Serialize)]
pub struct LoopBlock {
    pub condition: Option<String>,
    pub body: Vec<ProcessStmt>,
}

// ============================================================================
// Diagnostics (for when generation fails)
// ============================================================================

/// Diagnostic record produced when an emitter cannot generate valid DSL.
/// Captures enough context to debug and improve the generator offline.
#[derive(Debug, Clone, Serialize)]
pub struct DslDiagnostic {
    /// Which emitter failed
    pub emitter: String,
    /// Source file path
    pub source_file: String,
    /// COBOL program name
    pub program_name: String,
    /// What was being processed (function name, field name, etc.)
    pub context: String,
    /// Why generation failed
    pub reason: String,
    /// What the emitter saw in the Rust AST (abbreviated)
    pub ast_snippet: String,
    /// The #[cobol(...)] attributes that were present
    pub cobol_attrs: String,
    /// Suggested fix or investigation path
    pub suggestion: String,
}

/// Result of a typed emission attempt.
#[derive(Debug, Clone, Serialize)]
pub enum EmitResult<T> {
    /// Successfully produced a typed AST node
    Ok(T),
    /// Could not produce a valid AST node -- diagnostic explains why
    Diagnostic(DslDiagnostic),
}

impl<T> EmitResult<T> {
    pub fn ok(value: T) -> Self { Self::Ok(value) }
    pub fn fail(diag: DslDiagnostic) -> Self { Self::Diagnostic(diag) }

    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Ok(v) => Some(v),
            Self::Diagnostic(_) => None,
        }
    }

    pub fn diagnostic(&self) -> Option<&DslDiagnostic> {
        match self {
            Self::Ok(_) => None,
            Self::Diagnostic(d) => Some(d),
        }
    }
}

// ============================================================================
// Serializer: Typed AST -> grammar-valid DSL text
// ============================================================================

/// Indentation helper.
fn indent(level: usize) -> String {
    "    ".repeat(level)
}

impl SchemaFile {
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        for c in &self.comments {
            let _ = writeln!(out, "// {}", c.0);
        }
        if !self.comments.is_empty() { out.push('\n'); }
        for imp in &self.imports {
            let _ = writeln!(out, "import {}", imp.path);
        }
        if !self.imports.is_empty() { out.push('\n'); }
        for (i, schema) in self.schemas.iter().enumerate() {
            if i > 0 { out.push('\n'); }
            out.push_str(&schema.to_text());
        }
        out
    }
}

impl SchemaDef {
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "schema {}", self.name);

        if let Some(p) = &self.pattern {
            let _ = writeln!(out, "{}pattern {p}", indent(1));
            out.push('\n');
        }

        if let Some(id_fields) = &self.identity {
            let _ = writeln!(out, "{}identity", indent(1));
            for f in id_fields {
                f.write_to(&mut out, 2);
            }
            let _ = writeln!(out, "{}end", indent(1));
            out.push('\n');
        }

        if !self.fields.is_empty() {
            let _ = writeln!(out, "{}fields", indent(1));
            for f in &self.fields {
                f.write_to(&mut out, 2);
            }
            let _ = writeln!(out, "{}end", indent(1));
        }

        if !self.nested_objects.is_empty() {
            out.push('\n');
            for nested in &self.nested_objects {
                nested.write_to(&mut out, 1);
            }
        }

        if !self.constraints.is_empty() {
            out.push('\n');
            let _ = writeln!(out, "{}constraints", indent(1));
            for c in &self.constraints {
                match c {
                    SchemaConstraint::Enum(field, values) => {
                        let quoted: Vec<String> = values.iter().map(|v| format!("\"{v}\"")).collect();
                        let _ = writeln!(out, "{}{} enum({})", indent(2), field, quoted.join(", "));
                    }
                }
            }
            let _ = writeln!(out, "{}end", indent(1));
        }

        let _ = writeln!(out, "end");
        out
    }
}

impl NestedObjectDef {
    fn write_to(&self, out: &mut String, level: usize) {
        if let Some(c) = &self.comment {
            let _ = writeln!(out, "{}// {c}", indent(level));
        }
        if self.is_list {
            let _ = writeln!(out, "{}{} list<object>", indent(level), self.name);
        } else {
            let _ = writeln!(out, "{}{} object", indent(level), self.name);
        }
        for f in &self.fields {
            f.write_to(out, level + 1);
        }
        for nested in &self.nested {
            nested.write_to(out, level + 1);
        }
        let _ = writeln!(out, "{}end", indent(level));
    }
}

impl FieldDecl {
    fn write_to(&self, out: &mut String, level: usize) {
        if let Some(c) = &self.comment {
            let _ = writeln!(out, "{}// {c}", indent(level));
        }
        let req = if self.required { " required" } else { "" };
        let _ = writeln!(out, "{}{} {}{req}", indent(level), self.name, self.field_type);
    }
}

impl RulesFile {
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        for c in &self.comments {
            let _ = writeln!(out, "// {}", c.0);
        }
        if !self.comments.is_empty() { out.push('\n'); }
        for imp in &self.imports {
            let _ = writeln!(out, "import {}", imp.path);
        }
        if !self.imports.is_empty() { out.push('\n'); }
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 { out.push('\n'); }
            match item {
                RuleItem::DecisionTable(dt) => out.push_str(&dt.to_text()),
                RuleItem::ProceduralRule(pr) => out.push_str(&pr.to_text()),
            }
        }
        out
    }
}

impl DecisionTable {
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        if let Some(c) = &self.comment {
            let _ = writeln!(out, "// {c}");
        }
        let _ = writeln!(out, "decision_table {}", self.name);
        let _ = writeln!(out, "{}hit_policy {}", indent(1), self.hit_policy);
        if let Some(desc) = &self.description {
            let _ = writeln!(out, "{}description : \"{desc}\"", indent(1));
        }
        if let Some(ver) = &self.version {
            let _ = writeln!(out, "{}version : {ver}", indent(1));
        }
        out.push('\n');

        // Given block
        let _ = writeln!(out, "{}given:", indent(1));
        for p in &self.given {
            let _ = writeln!(out, "{}{} : {}", indent(2), p.name, p.param_type);
        }
        out.push('\n');

        // Decide block
        let _ = writeln!(out, "{}decide:", indent(1));

        // Header
        let mut header = format!("{}| {} | ", indent(2), self.decide.condition_col);
        for col in &self.decide.action_cols {
            let _ = write!(header, "{col} | ");
        }
        let _ = writeln!(out, "{header}");

        // Separator
        let sep_len = header.trim().len().max(20);
        let _ = writeln!(out, "{}|{}|", indent(2), "=".repeat(sep_len));

        // Rows
        for row in &self.decide.rows {
            let mut line = format!("{}| {} | ", indent(2), row.condition);
            for action in &row.actions {
                let _ = write!(line, "{action} | ");
            }
            let _ = writeln!(out, "{line}");
        }
        out.push('\n');

        // Return block
        let _ = writeln!(out, "{}return:", indent(1));
        for p in &self.return_params {
            let _ = writeln!(out, "{}{} : {}", indent(2), p.name, p.param_type);
        }

        // Post-calculate block
        if let Some(stmts) = &self.post_calculate {
            out.push('\n');
            let _ = writeln!(out, "{}post_calculate:", indent(1));
            for stmt in stmts {
                match stmt {
                    RuleStmt::Set(name, expr) => {
                        let _ = writeln!(out, "{}set {} = {}", indent(2), name, expr);
                    }
                    RuleStmt::Let(name, expr) => {
                        let _ = writeln!(out, "{}let {} = {}", indent(2), name, expr);
                    }
                    _ => {}
                }
            }
        }

        let _ = writeln!(out, "end");
        out
    }
}

impl ProceduralRule {
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        if let Some(c) = &self.comment {
            let _ = writeln!(out, "// {c}");
        }
        let _ = writeln!(out, "rule {}:", self.name);
        if let Some(d) = &self.description {
            let _ = writeln!(out, "{}description \"{d}\"", indent(1));
            out.push('\n');
        }
        for stmt in &self.body {
            write_rule_stmt(&mut out, stmt, 1);
        }
        let _ = writeln!(out, "end");
        out
    }
}

fn write_rule_stmt(out: &mut String, stmt: &RuleStmt, level: usize) {
    match stmt {
        RuleStmt::If(if_stmt) => {
            let _ = writeln!(out, "{}if {} then", indent(level), if_stmt.condition);
            for s in &if_stmt.then_block {
                write_rule_stmt(out, s, level + 1);
            }
            for (cond, block) in &if_stmt.elseif_blocks {
                let _ = writeln!(out, "{}elseif {} then", indent(level), cond);
                for s in block {
                    write_rule_stmt(out, s, level + 1);
                }
            }
            if let Some(else_block) = &if_stmt.else_block {
                let _ = writeln!(out, "{}else", indent(level));
                for s in else_block {
                    write_rule_stmt(out, s, level + 1);
                }
            }
            let _ = writeln!(out, "{}endif", indent(level));
        }
        RuleStmt::Set(field, expr) => {
            let _ = writeln!(out, "{}set {} = {}", indent(level), field, expr);
        }
        RuleStmt::Let(field, expr) => {
            let _ = writeln!(out, "{}let {} = {}", indent(level), field, expr);
        }
        RuleStmt::Return => {
            out.push('\n');
            let _ = writeln!(out, "{}return", indent(level));
        }
    }
}

impl TransformFile {
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        for c in &self.comments {
            let _ = writeln!(out, "// {}", c.0);
        }
        if !self.comments.is_empty() { out.push('\n'); }
        for imp in &self.imports {
            let _ = writeln!(out, "import {}", imp.path);
        }
        if !self.imports.is_empty() { out.push('\n'); }
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 { out.push('\n'); }
            match item {
                TransformItem::Transform(t) => out.push_str(&t.to_text()),
                TransformItem::TransformBlock(tb) => out.push_str(&tb.to_text()),
            }
        }
        out
    }
}

impl TransformDef {
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        if let Some(c) = &self.comment {
            let _ = writeln!(out, "// {c}");
        }
        let _ = writeln!(out, "transform {}", self.name);
        write_metadata(&mut out, &self.metadata, 1);
        let _ = writeln!(out, "{}pure : {}", indent(1), if self.pure { "true" } else { "false" });
        write_cache_decl(&mut out, &self.cache, 1);
        out.push('\n');
        write_io_spec(&mut out, "input", &self.input, 1);
        out.push('\n');
        write_io_spec(&mut out, "output", &self.output, 1);
        write_validation_block(&mut out, "validate_input", &self.validate_input, 1);
        out.push('\n');
        let _ = writeln!(out, "{}apply", indent(1));
        for stmt in &self.apply {
            match stmt {
                ApplyStmt::Assign(target, expr) => {
                    let _ = writeln!(out, "{}{} = {}", indent(2), target, expr);
                }
                ApplyStmt::Let(name, expr) => {
                    let _ = writeln!(out, "{}let {} = {}", indent(2), name, expr);
                }
            }
        }
        let _ = writeln!(out, "{}end", indent(1));
        write_validation_block(&mut out, "validate_output", &self.validate_output, 1);
        write_on_error_block(&mut out, &self.on_error, 1);
        let _ = writeln!(out, "end");
        out
    }
}

impl TransformBlockDef {
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        if let Some(c) = &self.comment {
            let _ = writeln!(out, "// {c}");
        }
        let _ = writeln!(out, "transform_block {}", self.name);
        write_metadata(&mut out, &self.metadata, 1);
        write_use_block(&mut out, &self.use_decls, 1);
        out.push('\n');
        write_io_spec(&mut out, "input", &self.input, 1);
        out.push('\n');
        write_io_spec(&mut out, "output", &self.output, 1);
        write_validation_block(&mut out, "validate_input", &self.validate_input, 1);
        out.push('\n');
        match &self.body {
            TransformBlockBody::Mappings(mappings) => {
                let _ = writeln!(out, "{}mappings", indent(1));
                for m in mappings {
                    let _ = writeln!(out, "{}{} = {}", indent(2), m.target, m.source);
                }
                let _ = writeln!(out, "{}end", indent(1));
            }
            TransformBlockBody::Compose(compose) => {
                let _ = writeln!(out, "{}compose {}", indent(1), compose.compose_type);
                for r in &compose.refs {
                    match r {
                        ComposeRef::Simple(id) => {
                            let _ = writeln!(out, "{}{id}", indent(2));
                        }
                        ComposeRef::When(expr, id) => {
                            let _ = writeln!(out, "{}when {expr} : {id}", indent(2));
                        }
                        ComposeRef::Otherwise(id) => {
                            let _ = writeln!(out, "{}otherwise : {id}", indent(2));
                        }
                    }
                }
                let _ = writeln!(out, "{}end", indent(1));
            }
        }
        write_validation_block(&mut out, "validate_output", &self.validate_output, 1);
        write_on_error_block(&mut out, &self.on_error, 1);
        let _ = writeln!(out, "end");
        out
    }
}

fn write_io_spec(out: &mut String, keyword: &str, spec: &IoSpec, level: usize) {
    match spec {
        IoSpec::Single(_name, ft) => {
            let _ = writeln!(out, "{}{keyword} : {ft}", indent(level));
        }
        IoSpec::Multi(fields) => {
            if fields.is_empty() {
                let _ = writeln!(out, "{}{keyword} : integer", indent(level));
            } else {
                let _ = writeln!(out, "{}{keyword}", indent(level));
                for f in fields {
                    let _ = writeln!(out, "{}{} : {}", indent(level + 1), f.name, f.field_type);
                }
                let _ = writeln!(out, "{}end", indent(level));
            }
        }
    }
}

fn write_metadata(out: &mut String, metadata: &Option<TransformMetadata>, level: usize) {
    let meta = match metadata {
        Some(m) => m,
        None => return,
    };
    if let Some(ver) = &meta.version {
        let _ = writeln!(out, "{}version : {ver}", indent(level));
    }
    if let Some(desc) = &meta.description {
        let _ = writeln!(out, "{}description : \"{desc}\"", indent(level));
    }
}

fn write_cache_decl(out: &mut String, cache: &Option<CacheDecl>, level: usize) {
    let c = match cache {
        Some(c) => c,
        None => return,
    };
    match (&c.ttl, &c.key) {
        (Some(ttl), None) => {
            let _ = writeln!(out, "{}cache : {ttl}", indent(level));
        }
        (ttl, key) => {
            let _ = writeln!(out, "{}cache", indent(level));
            if let Some(ttl) = ttl {
                let _ = writeln!(out, "{}ttl : {ttl}", indent(level + 1));
            }
            if let Some(fields) = key {
                let names: Vec<&str> = fields.iter().map(|f| f.as_str()).collect();
                let _ = writeln!(out, "{}key : [{}]", indent(level + 1), names.join(", "));
            }
            let _ = writeln!(out, "{}end", indent(level));
        }
    }
}

fn write_use_block(out: &mut String, use_decls: &Option<Vec<Ident>>, level: usize) {
    let decls = match use_decls {
        Some(d) if !d.is_empty() => d,
        _ => return,
    };
    let _ = write!(out, "{}use", indent(level));
    for d in decls {
        let _ = write!(out, " {d}");
    }
    let _ = writeln!(out, " end");
}

fn write_validation_block(
    out: &mut String,
    keyword: &str,
    rules: &Option<Vec<ValidationRule>>,
    level: usize,
) {
    let rules = match rules {
        Some(r) if !r.is_empty() => r,
        _ => return,
    };
    out.push('\n');
    let _ = writeln!(out, "{}{keyword}", indent(level));
    for rule in rules {
        match rule {
            ValidationRule::Require(expr, msg) => {
                let _ = write!(out, "{}require {} else \"{}\"", indent(level + 1), expr, msg.text);
                if let Some(code) = &msg.code {
                    let _ = write!(out, " code : \"{code}\"");
                }
                if let Some(sev) = &msg.severity {
                    let _ = write!(out, " severity : {sev}");
                }
                let _ = writeln!(out);
            }
            ValidationRule::Simple(expr, msg) => {
                let _ = writeln!(out, "{}{} : \"{}\"", indent(level + 1), expr, msg.text);
            }
        }
    }
    let _ = writeln!(out, "{}end", indent(level));
}

fn write_on_error_block(
    out: &mut String,
    stmts: &Option<Vec<ErrorStatement>>,
    level: usize,
) {
    let stmts = match stmts {
        Some(s) if !s.is_empty() => s,
        _ => return,
    };
    out.push('\n');
    let _ = writeln!(out, "{}on_error", indent(level));
    for stmt in stmts {
        match stmt {
            ErrorStatement::Action(action) => {
                let _ = writeln!(out, "{}action : {action}", indent(level + 1));
            }
            ErrorStatement::LogError(msg) => {
                let _ = writeln!(out, "{}log_error(\"{msg}\")", indent(level + 1));
            }
            ErrorStatement::ErrorCode(code) => {
                let _ = writeln!(out, "{}error_code : \"{code}\"", indent(level + 1));
            }
            ErrorStatement::Emit(mode) => {
                let mode_str = match mode {
                    EmitMode::Defaults => "defaults",
                    EmitMode::Partial => "partial",
                };
                let _ = writeln!(out, "{}emit with {mode_str}", indent(level + 1));
            }
            ErrorStatement::Reject(msg) => {
                let _ = writeln!(out, "{}reject with \"{msg}\"", indent(level + 1));
            }
            ErrorStatement::Default(expr) => {
                let _ = writeln!(out, "{}default : {expr}", indent(level + 1));
            }
        }
    }
    let _ = writeln!(out, "{}end", indent(level));
}

impl ProcessFile {
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        for c in &self.comments {
            let _ = writeln!(out, "// {}", c.0);
        }
        if !self.comments.is_empty() { out.push('\n'); }
        for imp in &self.imports {
            let _ = writeln!(out, "import {}", imp.path);
        }
        if !self.imports.is_empty() { out.push('\n'); }
        for proc in &self.processes {
            out.push_str(&proc.to_text());
        }
        out
    }
}

impl ProcessDef {
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "process {}", self.name);
        if let Some(mode) = &self.mode {
            let _ = writeln!(out, "{}mode {mode}", indent(1));
        }
        if let Some(exec) = &self.execution {
            if let Some(p) = exec.parallelism {
                let _ = writeln!(out, "{}parallelism {p}", indent(1));
            }
            if let Some(fields) = &exec.partition_by {
                let names: Vec<&str> = fields.iter().map(|f| f.as_str()).collect();
                let _ = writeln!(out, "{}partition by {}", indent(1), names.join(", "));
            }
        }
        out.push('\n');
        for stmt in &self.body {
            write_process_stmt(&mut out, stmt, 1);
        }
        out.push('\n');
        let _ = writeln!(out, "end");
        out
    }
}

fn write_process_stmt(out: &mut String, stmt: &ProcessStmt, level: usize) {
    match stmt {
        ProcessStmt::Comment(c) => {
            let _ = writeln!(out, "{}// {c}", indent(level));
        }
        ProcessStmt::Receive(r) => {
            let _ = write!(out, "{}receive {}", indent(level), r.name);
            if let Some(conn) = &r.connector {
                let _ = write!(out, " from {}", conn.connector_type);
                if let Some(cfg) = &conn.config {
                    let _ = write!(out, " \"{cfg}\"");
                }
            }
            let _ = writeln!(out);
            if let Some(s) = &r.schema {
                let _ = writeln!(out, "{}schema {s}", indent(level + 1));
            }
        }
        ProcessStmt::TransformUsing { input, using, output } => {
            let _ = writeln!(out, "{}transform {} using {} into {}",
                indent(level), input, using, output);
        }
        ProcessStmt::EvaluateUsing(name) => {
            let _ = writeln!(out, "{}evaluate using {name}", indent(level));
        }
        ProcessStmt::Emit(e) => {
            let _ = write!(out, "{}emit to {}", indent(level), e.target);
            if let Some(conn) = &e.connector {
                let _ = write!(out, " {}", conn.connector_type);
                if let Some(cfg) = &conn.config {
                    let _ = write!(out, " \"{cfg}\"");
                }
            }
            let _ = writeln!(out);
            if let Some(s) = &e.schema {
                let _ = writeln!(out, "{}schema {s}", indent(level + 1));
            }
        }
        ProcessStmt::Route(r) => {
            let _ = writeln!(out, "{}route using {}", indent(level), r.using);
            for (cond, target) in &r.destinations {
                if let Some(c) = cond {
                    let _ = writeln!(out, "{}\"{c}\" to {target}", indent(level + 1));
                } else {
                    let _ = writeln!(out, "{}to {target}", indent(level + 1));
                }
            }
            if let Some(otherwise) = &r.otherwise {
                let _ = writeln!(out, "{}otherwise to {otherwise}", indent(level + 1));
            }
        }
        ProcessStmt::Parallel(p) => {
            let _ = writeln!(out, "{}parallel", indent(level));
            for (name, body) in &p.branches {
                let _ = writeln!(out, "{}branch {name}", indent(level + 1));
                for s in body {
                    write_process_stmt(out, s, level + 2);
                }
                let _ = writeln!(out, "{}end", indent(level + 1));
            }
            let _ = writeln!(out, "{}end", indent(level));
        }
        ProcessStmt::Loop(l) => {
            let _ = writeln!(out, "{}loop", indent(level));
            if let Some(cond) = &l.condition {
                let _ = writeln!(out, "{}// until: {cond}", indent(level + 1));
            }
            for s in &l.body {
                write_process_stmt(out, s, level + 1);
            }
            let _ = writeln!(out, "{}end", indent(level));
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ident_sanitizes() {
        assert_eq!(Ident::new("WS-ACCT-NUMBER").as_str(), "ws_acct_number");
        assert_eq!(Ident::new("123bad").as_str(), "_123bad");
        assert_eq!(Ident::new("").as_str(), "unnamed");
        assert_eq!(Ident::new("ok_name").as_str(), "ok_name");
    }

    #[test]
    fn schema_round_trip() {
        let schema = SchemaFile {
            comments: vec![Comment("Generated by cobol2rust".to_string())],
            imports: vec![],
            schemas: vec![SchemaDef {
                name: Ident::new("account_info"),
                pattern: Some(MutationPattern::MasterData),
                identity: Some(vec![FieldDecl {
                    name: Ident::new("acct_number"),
                    field_type: FieldType::String(Some(10)),
                    required: true,
                    comment: None,
                }]),
                fields: vec![
                    FieldDecl {
                        name: Ident::new("acct_number"),
                        field_type: FieldType::String(Some(10)),
                        required: true,
                        comment: Some("COBOL: WS-ACCT-NUMBER PIC X(10)".to_string()),
                    },
                    FieldDecl {
                        name: Ident::new("balance"),
                        field_type: FieldType::Decimal(11, 2),
                        required: true,
                        comment: None,
                    },
                ],
                nested_objects: vec![],
                constraints: vec![SchemaConstraint::Enum(
                    Ident::new("acct_type"),
                    vec!["S".to_string(), "C".to_string()],
                )],
            }],
        };

        let text = schema.to_text();
        assert!(text.contains("schema account_info"));
        assert!(text.contains("pattern master_data"));
        assert!(text.contains("identity"));
        assert!(text.contains("fields"));
        assert!(text.contains("string(10)"));
        assert!(text.contains("decimal(11, 2)"));
        assert!(text.contains("constraints"));
        assert!(text.contains("enum(\"S\", \"C\")"));
        assert!(text.contains("end"));
        // Count: schema has identity end, fields end, constraints end, schema end = 4
        let end_count = text.lines().filter(|l| l.trim() == "end").count();
        assert!(end_count >= 4, "Should have at least 4 'end' lines, got {end_count}");
    }

    #[test]
    fn decision_table_round_trip() {
        let dt = DecisionTable {
            name: Ident::new("rate_lookup"),
            comment: Some("COBOL paragraph: RATE-PARA".to_string()),
            hit_policy: HitPolicy::FirstMatch,
            description: None,
            version: None,
            given: vec![GivenParam {
                name: Ident::new("ws_a"),
                param_type: RulesParamType::Number,
            }],
            decide: DecideMatrix {
                condition_col: "condition".to_string(),
                action_cols: vec![Ident::new("ws_result")],
                rows: vec![
                    DecideRow { condition: "ws_a > 75".to_string(), actions: vec!["\"HIGH\"".to_string()] },
                    DecideRow { condition: "ws_a > 50".to_string(), actions: vec!["\"MEDIUM\"".to_string()] },
                    DecideRow { condition: "otherwise".to_string(), actions: vec!["\"LOW\"".to_string()] },
                ],
            },
            return_params: vec![ReturnParam {
                name: Ident::new("ws_result"),
                param_type: RulesParamType::Text,
            }],
            post_calculate: None,
        };

        let text = dt.to_text();
        assert!(text.contains("decision_table rate_lookup"));
        assert!(text.contains("hit_policy first_match"));
        assert!(text.contains("given:"));
        assert!(text.contains("ws_a : number"));
        assert!(text.contains("decide:"));
        assert!(text.contains("ws_a > 75"));
        assert!(text.contains("\"HIGH\""));
        assert!(text.contains("otherwise"));
        assert!(text.contains("return:"));
        assert!(text.contains("end"));
    }

    #[test]
    fn decision_table_with_description_version() {
        let dt = DecisionTable {
            name: Ident::new("rate_calc"),
            comment: None,
            hit_policy: HitPolicy::FirstMatch,
            description: Some("Calculate interest rate based on account type".to_string()),
            version: Some("1.0.0".to_string()),
            given: vec![GivenParam {
                name: Ident::new("acct_type"),
                param_type: RulesParamType::Text,
            }],
            decide: DecideMatrix {
                condition_col: "acct_type".to_string(),
                action_cols: vec![Ident::new("rate")],
                rows: vec![
                    DecideRow { condition: "\"S\"".to_string(), actions: vec!["3.5".to_string()] },
                ],
            },
            return_params: vec![ReturnParam {
                name: Ident::new("rate"),
                param_type: RulesParamType::Number,
            }],
            post_calculate: None,
        };

        let text = dt.to_text();
        assert!(text.contains("description : \"Calculate interest rate"), "Should have description: {text}");
        assert!(text.contains("version : 1.0.0"), "Should have version: {text}");
    }

    #[test]
    fn decision_table_with_post_calculate() {
        let dt = DecisionTable {
            name: Ident::new("tax_calc"),
            comment: None,
            hit_policy: HitPolicy::FirstMatch,
            description: None,
            version: None,
            given: vec![GivenParam {
                name: Ident::new("income"),
                param_type: RulesParamType::Money,
            }],
            decide: DecideMatrix {
                condition_col: "income".to_string(),
                action_cols: vec![Ident::new("rate")],
                rows: vec![
                    DecideRow { condition: "> 100000".to_string(), actions: vec!["0.30".to_string()] },
                ],
            },
            return_params: vec![ReturnParam {
                name: Ident::new("tax"),
                param_type: RulesParamType::Money,
            }],
            post_calculate: Some(vec![
                RuleStmt::Let(
                    Ident::new("tax"),
                    Expr::Binary(
                        Box::new(Expr::Field(Ident::new("income"))),
                        BinOp::Mul,
                        Box::new(Expr::Field(Ident::new("rate"))),
                    ),
                ),
            ]),
        };

        let text = dt.to_text();
        assert!(text.contains("post_calculate:"), "Should have post_calculate: {text}");
        assert!(text.contains("let tax = income * rate"), "Should have let statement: {text}");
    }

    #[test]
    fn decision_table_no_optional_fields() {
        let dt = DecisionTable {
            name: Ident::new("simple"),
            comment: None,
            hit_policy: HitPolicy::FirstMatch,
            description: None,
            version: None,
            given: vec![],
            decide: DecideMatrix {
                condition_col: "x".to_string(),
                action_cols: vec![],
                rows: vec![],
            },
            return_params: vec![],
            post_calculate: None,
        };

        let text = dt.to_text();
        assert!(!text.contains("description"), "Should NOT have description: {text}");
        assert!(!text.contains("version"), "Should NOT have version: {text}");
        assert!(!text.contains("post_calculate"), "Should NOT have post_calculate: {text}");
    }

    #[test]
    fn procedural_rule_round_trip() {
        let rule = ProceduralRule {
            name: Ident::new("check_score"),
            comment: None,
            description: Some("Check score thresholds".to_string()),
            body: vec![
                RuleStmt::If(IfStmt {
                    condition: Expr::Binary(
                        Box::new(Expr::Field(Ident::new("ws_a"))),
                        BinOp::Gt,
                        Box::new(Expr::Lit(Literal::Int(75))),
                    ),
                    then_block: vec![RuleStmt::Set(
                        Ident::new("ws_result"),
                        Expr::Lit(Literal::Str("HIGH".to_string())),
                    )],
                    elseif_blocks: vec![(
                        Expr::Binary(
                            Box::new(Expr::Field(Ident::new("ws_a"))),
                            BinOp::Gt,
                            Box::new(Expr::Lit(Literal::Int(50))),
                        ),
                        vec![RuleStmt::Set(
                            Ident::new("ws_result"),
                            Expr::Lit(Literal::Str("MEDIUM".to_string())),
                        )],
                    )],
                    else_block: Some(vec![RuleStmt::Set(
                        Ident::new("ws_result"),
                        Expr::Lit(Literal::Str("LOW".to_string())),
                    )]),
                }),
                RuleStmt::Return,
            ],
        };

        let text = rule.to_text();
        assert!(text.contains("rule check_score:"));
        assert!(text.contains("description \"Check score thresholds\""));
        assert!(text.contains("if ws_a > 75 then"));
        assert!(text.contains("set ws_result = \"HIGH\""));
        assert!(text.contains("elseif ws_a > 50 then"));
        assert!(text.contains("set ws_result = \"MEDIUM\""));
        assert!(text.contains("else"));
        assert!(text.contains("set ws_result = \"LOW\""));
        assert!(text.contains("endif"));
        assert!(text.contains("return"));
        assert!(text.contains("end"));
    }

    #[test]
    fn transform_compose_round_trip() {
        let tb = TransformBlockDef {
            name: Ident::new("processing_section"),
            comment: Some("COBOL section: PROCESSING-SECTION".to_string()),
            metadata: None,
            use_decls: None,
            input: IoSpec::Single(Ident::new("input"), FieldType::Integer(None)),
            output: IoSpec::Single(Ident::new("output"), FieldType::Integer(None)),
            body: TransformBlockBody::Compose(ComposeBlock {
                compose_type: ComposeType::Sequential,
                refs: vec![
                    ComposeRef::Simple(Ident::new("init_para")),
                    ComposeRef::Simple(Ident::new("calc_para")),
                    ComposeRef::Simple(Ident::new("finish_para")),
                ],
            }),
            validate_input: None,
            validate_output: None,
            on_error: None,
        };

        let text = tb.to_text();
        assert!(text.contains("transform_block processing_section"));
        assert!(text.contains("compose sequential"));
        assert!(text.contains("init_para"));
        assert!(text.contains("calc_para"));
        assert!(text.contains("finish_para"));
        assert!(text.contains("end"));
    }

    #[test]
    fn process_round_trip() {
        let proc = ProcessFile {
            comments: vec![Comment("Generated by cobol2rust".to_string())],
            imports: vec![
                ImportPath::schema("TESTPROG"),
                ImportPath::transform("processing_section"),
            ],
            processes: vec![ProcessDef {
                name: Ident::new("testprog"),
                mode: Some(ProcessMode::Batch),
                execution: None,
                body: vec![
                    ProcessStmt::Receive(ReceiveBlock {
                        name: Ident::new("input_records"),
                        schema: Some(Ident::new("testprog_input")),
                        connector: None,
                    }),
                    ProcessStmt::Comment("Entry: RUN".to_string()),
                    ProcessStmt::TransformUsing {
                        input: Ident::new("input_records"),
                        using: Ident::new("calc_para"),
                        output: Ident::new("output_records"),
                    },
                    ProcessStmt::EvaluateUsing(Ident::new("decide_para")),
                    ProcessStmt::Emit(EmitBlock {
                        target: Ident::new("output_records"),
                        schema: Some(Ident::new("testprog_output")),
                        connector: None,
                    }),
                ],
            }],
        };

        let text = proc.to_text();
        assert!(text.contains("process testprog"));
        assert!(text.contains("mode batch"));
        assert!(text.contains("receive input_records"));
        assert!(text.contains("schema testprog_input"));
        assert!(text.contains("transform input_records using calc_para into output_records"));
        assert!(text.contains("evaluate using decide_para"));
        assert!(text.contains("emit to output_records"));
        assert!(text.contains("end"));
    }

    #[test]
    fn on_error_block_serialization() {
        let td = TransformDef {
            name: Ident::new("calc_para"),
            comment: None,
            metadata: None,
            pure: true,
            cache: None,
            input: IoSpec::Single(Ident::new("input"), FieldType::Integer(None)),
            output: IoSpec::Single(Ident::new("result"), FieldType::Integer(None)),
            apply: vec![ApplyStmt::Assign(
                Ident::new("result"),
                Expr::Binary(
                    Box::new(Expr::Field(Ident::new("a"))),
                    BinOp::Add,
                    Box::new(Expr::Field(Ident::new("b"))),
                ),
            )],
            validate_input: None,
            validate_output: None,
            on_error: Some(vec![
                ErrorStatement::Action(ErrorAction::Raise),
                ErrorStatement::LogError("ON SIZE ERROR in COMPUTE".to_string()),
                ErrorStatement::ErrorCode("SIZE_ERROR".to_string()),
            ]),
        };

        let text = td.to_text();
        assert!(text.contains("on_error"), "Should have on_error block: {text}");
        assert!(text.contains("action : raise"), "Should have action: {text}");
        assert!(text.contains("log_error(\"ON SIZE ERROR in COMPUTE\")"), "Should have log: {text}");
        assert!(text.contains("error_code : \"SIZE_ERROR\""), "Should have code: {text}");
        // on_error block should be properly terminated
        let on_error_pos = text.find("on_error").unwrap();
        let after_on_error = &text[on_error_pos..];
        assert!(after_on_error.contains("end"), "on_error block should have end: {after_on_error}");
    }

    #[test]
    fn validation_block_serialization() {
        let td = TransformDef {
            name: Ident::new("validate_para"),
            comment: None,
            metadata: None,
            pure: true,
            cache: None,
            input: IoSpec::Single(Ident::new("amount"), FieldType::Decimal(9, 2)),
            output: IoSpec::Single(Ident::new("result"), FieldType::Integer(None)),
            apply: vec![ApplyStmt::Assign(
                Ident::new("result"),
                Expr::Field(Ident::new("amount")),
            )],
            validate_input: Some(vec![
                ValidationRule::Require(
                    Expr::Binary(
                        Box::new(Expr::Field(Ident::new("amount"))),
                        BinOp::Ge,
                        Box::new(Expr::Lit(Literal::Int(0))),
                    ),
                    ValidationMessage {
                        text: "Amount must be non-negative".to_string(),
                        code: None,
                        severity: None,
                    },
                ),
            ]),
            validate_output: None,
            on_error: None,
        };

        let text = td.to_text();
        assert!(text.contains("validate_input"), "Should have validate_input: {text}");
        assert!(text.contains("require amount >= 0 else"), "Should have require rule: {text}");
        assert!(text.contains("Amount must be non-negative"), "Should have message: {text}");
    }

    #[test]
    fn conditional_compose_serialization() {
        let tb = TransformBlockDef {
            name: Ident::new("dispatch_para"),
            comment: None,
            metadata: None,
            use_decls: None,
            input: IoSpec::Single(Ident::new("status"), FieldType::String(Some(1))),
            output: IoSpec::Single(Ident::new("result"), FieldType::Integer(None)),
            body: TransformBlockBody::Compose(ComposeBlock {
                compose_type: ComposeType::Conditional,
                refs: vec![
                    ComposeRef::When(
                        Expr::Binary(
                            Box::new(Expr::Field(Ident::new("status"))),
                            BinOp::Eq,
                            Box::new(Expr::Lit(Literal::Str("A".to_string()))),
                        ),
                        Ident::new("process_active"),
                    ),
                    ComposeRef::When(
                        Expr::Binary(
                            Box::new(Expr::Field(Ident::new("status"))),
                            BinOp::Eq,
                            Box::new(Expr::Lit(Literal::Str("C".to_string()))),
                        ),
                        Ident::new("process_closed"),
                    ),
                    ComposeRef::Otherwise(Ident::new("process_default")),
                ],
            }),
            validate_input: None,
            validate_output: None,
            on_error: None,
        };

        let text = tb.to_text();
        assert!(text.contains("compose conditional"), "Should have conditional: {text}");
        assert!(text.contains("when status == \"A\" : process_active"), "Should have when A: {text}");
        assert!(text.contains("when status == \"C\" : process_closed"), "Should have when C: {text}");
        assert!(text.contains("otherwise : process_default"), "Should have otherwise: {text}");
    }

    #[test]
    fn simple_compose_backward_compat() {
        let tb = TransformBlockDef {
            name: Ident::new("seq_para"),
            comment: None,
            metadata: None,
            use_decls: None,
            input: IoSpec::Single(Ident::new("x"), FieldType::Integer(None)),
            output: IoSpec::Single(Ident::new("y"), FieldType::Integer(None)),
            body: TransformBlockBody::Compose(ComposeBlock {
                compose_type: ComposeType::Sequential,
                refs: vec![
                    ComposeRef::Simple(Ident::new("step_a")),
                    ComposeRef::Simple(Ident::new("step_b")),
                ],
            }),
            validate_input: None,
            validate_output: None,
            on_error: None,
        };

        let text = tb.to_text();
        assert!(text.contains("compose sequential"), "Should have sequential: {text}");
        assert!(text.contains("step_a"), "Should have step_a: {text}");
        assert!(text.contains("step_b"), "Should have step_b: {text}");
        // Should NOT have when/otherwise
        assert!(!text.contains("when "), "Sequential should not have when: {text}");
    }

    #[test]
    fn no_on_error_no_block() {
        let td = TransformDef {
            name: Ident::new("simple"),
            comment: None,
            metadata: None,
            pure: true,
            cache: None,
            input: IoSpec::Single(Ident::new("x"), FieldType::Integer(None)),
            output: IoSpec::Single(Ident::new("y"), FieldType::Integer(None)),
            apply: vec![ApplyStmt::Assign(Ident::new("y"), Expr::Field(Ident::new("x")))],
            validate_input: None,
            validate_output: None,
            on_error: None,
        };

        let text = td.to_text();
        assert!(!text.contains("on_error"), "Should NOT have on_error: {text}");
        assert!(!text.contains("validate_input"), "Should NOT have validate_input: {text}");
    }

    #[test]
    fn metadata_serialization() {
        let td = TransformDef {
            name: Ident::new("versioned_transform"),
            comment: None,
            metadata: Some(TransformMetadata {
                version: Some("1.2.3".to_string()),
                description: Some("Calculate account balance".to_string()),
            }),
            pure: true,
            cache: None,
            input: IoSpec::Single(Ident::new("x"), FieldType::Integer(None)),
            output: IoSpec::Single(Ident::new("y"), FieldType::Integer(None)),
            apply: vec![ApplyStmt::Assign(Ident::new("y"), Expr::Field(Ident::new("x")))],
            validate_input: None,
            validate_output: None,
            on_error: None,
        };

        let text = td.to_text();
        assert!(text.contains("version : 1.2.3"), "Should have version: {text}");
        assert!(text.contains("description : \"Calculate account balance\""), "Should have description: {text}");
    }

    #[test]
    fn cache_short_form_serialization() {
        let td = TransformDef {
            name: Ident::new("cached_transform"),
            comment: None,
            metadata: None,
            pure: true,
            cache: Some(CacheDecl {
                ttl: Some("5 minutes".to_string()),
                key: None,
            }),
            input: IoSpec::Single(Ident::new("x"), FieldType::Integer(None)),
            output: IoSpec::Single(Ident::new("y"), FieldType::Integer(None)),
            apply: vec![ApplyStmt::Assign(Ident::new("y"), Expr::Field(Ident::new("x")))],
            validate_input: None,
            validate_output: None,
            on_error: None,
        };

        let text = td.to_text();
        assert!(text.contains("cache : 5 minutes"), "Should have short-form cache: {text}");
    }

    #[test]
    fn cache_long_form_serialization() {
        let td = TransformDef {
            name: Ident::new("cached_keyed"),
            comment: None,
            metadata: None,
            pure: true,
            cache: Some(CacheDecl {
                ttl: Some("1 hours".to_string()),
                key: Some(vec![Ident::new("account_id"), Ident::new("region")]),
            }),
            input: IoSpec::Single(Ident::new("x"), FieldType::Integer(None)),
            output: IoSpec::Single(Ident::new("y"), FieldType::Integer(None)),
            apply: vec![ApplyStmt::Assign(Ident::new("y"), Expr::Field(Ident::new("x")))],
            validate_input: None,
            validate_output: None,
            on_error: None,
        };

        let text = td.to_text();
        assert!(text.contains("cache"), "Should have cache block: {text}");
        assert!(text.contains("ttl : 1 hours"), "Should have ttl: {text}");
        assert!(text.contains("key : [account_id, region]"), "Should have key: {text}");
    }

    #[test]
    fn use_block_serialization() {
        let tb = TransformBlockDef {
            name: Ident::new("composed_transform"),
            comment: None,
            metadata: None,
            use_decls: Some(vec![
                Ident::new("helper_a"),
                Ident::new("helper_b"),
            ]),
            input: IoSpec::Single(Ident::new("x"), FieldType::Integer(None)),
            output: IoSpec::Single(Ident::new("y"), FieldType::Integer(None)),
            body: TransformBlockBody::Compose(ComposeBlock {
                compose_type: ComposeType::Sequential,
                refs: vec![
                    ComposeRef::Simple(Ident::new("helper_a")),
                    ComposeRef::Simple(Ident::new("helper_b")),
                ],
            }),
            validate_input: None,
            validate_output: None,
            on_error: None,
        };

        let text = tb.to_text();
        assert!(text.contains("use helper_a helper_b end"), "Should have use block: {text}");
    }

    #[test]
    fn no_metadata_no_cache_no_use() {
        let td = TransformDef {
            name: Ident::new("bare"),
            comment: None,
            metadata: None,
            pure: true,
            cache: None,
            input: IoSpec::Single(Ident::new("x"), FieldType::Integer(None)),
            output: IoSpec::Single(Ident::new("y"), FieldType::Integer(None)),
            apply: vec![ApplyStmt::Assign(Ident::new("y"), Expr::Field(Ident::new("x")))],
            validate_input: None,
            validate_output: None,
            on_error: None,
        };

        let text = td.to_text();
        assert!(!text.contains("version"), "Should NOT have version: {text}");
        assert!(!text.contains("description"), "Should NOT have description: {text}");
        assert!(!text.contains("cache"), "Should NOT have cache: {text}");
    }

    #[test]
    fn diagnostic_captures_failure() {
        let diag = DslDiagnostic {
            emitter: "rules".to_string(),
            source_file: "src/cardproc.rs".to_string(),
            program_name: "CARDPROC".to_string(),
            context: "function evaluate_para".to_string(),
            reason: "Unsupported match arm pattern: tuple struct".to_string(),
            ast_snippet: "match (ws.field_a, ws.field_b) { ... }".to_string(),
            cobol_attrs: "reads = WS-A,WS-B, writes = WS-C".to_string(),
            suggestion: "Add tuple pattern support to expr_extract.rs".to_string(),
        };

        let json = serde_json::to_string_pretty(&diag).unwrap();
        assert!(json.contains("Unsupported match arm pattern"));
        assert!(json.contains("evaluate_para"));
        assert!(json.contains("Add tuple pattern support"));
    }
}
