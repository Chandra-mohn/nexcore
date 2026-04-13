/// NexQuery Abstract Syntax Tree.
///
/// A script is a sequence of statements, each terminated by `;`.
/// A statement (query) is a sequence of clauses separated by newlines.
/// Clauses are: traverse, expand, or domain verb.

#[derive(Debug, Clone, PartialEq)]
pub struct Script {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub clauses: Vec<Clause>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Clause {
    Traverse(TraverseClause),
    Expand(ExpandClause),
    Verb(VerbClause),
}

/// `<node-type> <traversal-verb> <target> <filter?>`
#[derive(Debug, Clone, PartialEq)]
pub struct TraverseClause {
    pub node_type: NodeType,
    pub verb: TraversalVerb,
    pub target: Target,
    pub filter: Option<Filter>,
}

/// `<node-type> <filter?>` -- expand a dimension from the previous result set.
#[derive(Debug, Clone, PartialEq)]
pub struct ExpandClause {
    pub node_type: NodeType,
    pub filter: Option<Filter>,
}

/// `<domain-verb> <target?> <modifier*>`
#[derive(Debug, Clone, PartialEq)]
pub struct VerbClause {
    pub verb: DomainVerb,
    pub target: Option<Target>,
    pub modifiers: Vec<Modifier>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Programs,
    Paragraphs,
    Fields,
    Copybooks,
    Files,
    Tables,
    Rules,
}

impl NodeType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "programs" => Some(Self::Programs),
            "paragraphs" => Some(Self::Paragraphs),
            "fields" => Some(Self::Fields),
            "copybooks" => Some(Self::Copybooks),
            "files" => Some(Self::Files),
            "tables" => Some(Self::Tables),
            "rules" => Some(Self::Rules),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Programs => "programs",
            Self::Paragraphs => "paragraphs",
            Self::Fields => "fields",
            Self::Copybooks => "copybooks",
            Self::Files => "files",
            Self::Tables => "tables",
            Self::Rules => "rules",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraversalVerb {
    Calling,
    CalledBy,
    Performing,
    PerformedBy,
    Reading,
    ReadBy,
    Writing,
    WrittenBy,
    Using,
    UsedBy,
    Accessing,
    AccessedBy,
    Containing,
    Within,
}

impl TraversalVerb {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "calling" => Some(Self::Calling),
            "called-by" => Some(Self::CalledBy),
            "performing" => Some(Self::Performing),
            "performed-by" => Some(Self::PerformedBy),
            "reading" => Some(Self::Reading),
            "read-by" => Some(Self::ReadBy),
            "writing" => Some(Self::Writing),
            "written-by" => Some(Self::WrittenBy),
            "using" => Some(Self::Using),
            "used-by" => Some(Self::UsedBy),
            "accessing" => Some(Self::Accessing),
            "accessed-by" => Some(Self::AccessedBy),
            "containing" => Some(Self::Containing),
            "within" => Some(Self::Within),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Calling => "calling",
            Self::CalledBy => "called-by",
            Self::Performing => "performing",
            Self::PerformedBy => "performed-by",
            Self::Reading => "reading",
            Self::ReadBy => "read-by",
            Self::Writing => "writing",
            Self::WrittenBy => "written-by",
            Self::Using => "using",
            Self::UsedBy => "used-by",
            Self::Accessing => "accessing",
            Self::AccessedBy => "accessed-by",
            Self::Containing => "containing",
            Self::Within => "within",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainVerb {
    Trace,
    Rank,
    Similar,
    FindDead,
    Deps,
    Impact,
    Compare,
    DiscoverProcesses,
    EstimateCost,
    Save,
    Run,
}

impl DomainVerb {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "trace" => Some(Self::Trace),
            "rank" => Some(Self::Rank),
            "similar" => Some(Self::Similar),
            "find-dead" => Some(Self::FindDead),
            "deps" => Some(Self::Deps),
            "impact" => Some(Self::Impact),
            "compare" => Some(Self::Compare),
            "discover-processes" => Some(Self::DiscoverProcesses),
            "estimate-cost" => Some(Self::EstimateCost),
            "save" => Some(Self::Save),
            "run" => Some(Self::Run),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Trace => "trace",
            Self::Rank => "rank",
            Self::Similar => "similar",
            Self::FindDead => "find-dead",
            Self::Deps => "deps",
            Self::Impact => "impact",
            Self::Compare => "compare",
            Self::DiscoverProcesses => "discover-processes",
            Self::EstimateCost => "estimate-cost",
            Self::Save => "save",
            Self::Run => "run",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    Identifier(String),
    Each,
    List(Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Filter {
    pub predicates: Vec<FilterExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FilterExpr {
    Predicate(Predicate),
    And(Box<FilterExpr>, Box<FilterExpr>),
    Or(Box<FilterExpr>, Box<FilterExpr>),
    Not(Box<FilterExpr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Predicate {
    pub field: String,
    pub op: CompareOp,
    pub value: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOp {
    Eq,
    NotEq,
    Gt,
    Lt,
    Gte,
    Lte,
    Glob,
    Regex,
    In,
    Has,
}

impl CompareOp {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Eq => "=",
            Self::NotEq => "!=",
            Self::Gt => ">",
            Self::Lt => "<",
            Self::Gte => ">=",
            Self::Lte => "<=",
            Self::Glob => "~",
            Self::Regex => "~~",
            Self::In => "in",
            Self::Has => "has",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    List(Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Modifier {
    pub keyword: ModifierKeyword,
    pub value: ModifierValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModifierKeyword {
    By,
    Top,
    In,
    With,
    Depth,
    Level,
    Order,
    Scope,
    Threshold,
    As,
}

impl ModifierKeyword {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "by" => Some(Self::By),
            "top" => Some(Self::Top),
            "in" => Some(Self::In),
            "with" => Some(Self::With),
            "depth" => Some(Self::Depth),
            "level" => Some(Self::Level),
            "order" => Some(Self::Order),
            "scope" => Some(Self::Scope),
            "threshold" => Some(Self::Threshold),
            "as" => Some(Self::As),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModifierValue {
    Identifier(String),
    Number(f64),
    NodeType(NodeType),
}
