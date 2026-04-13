use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type NodeId = petgraph::graph::NodeIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeKind {
    Program,
    Paragraph,
    Field,
    Copybook,
    File,
    Table,
    Rule,
}

impl NodeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Program => "program",
            Self::Paragraph => "paragraph",
            Self::Field => "field",
            Self::Copybook => "copybook",
            Self::File => "file",
            Self::Table => "table",
            Self::Rule => "rule",
        }
    }
}

impl std::fmt::Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub kind: NodeKind,
    pub name: String,
    pub program: Option<String>,
    pub properties: Properties,
}

impl Node {
    pub fn new(kind: NodeKind, name: impl Into<String>) -> Self {
        Self {
            kind,
            name: name.into(),
            program: None,
            properties: Properties::default(),
        }
    }

    pub fn with_program(mut self, program: impl Into<String>) -> Self {
        self.program = Some(program.into());
        self
    }

    pub fn with_property(mut self, key: impl Into<String>, value: PropValue) -> Self {
        self.properties.set(key, value);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Properties {
    values: HashMap<String, PropValue>,
}

impl Properties {
    pub fn get(&self, key: &str) -> Option<&PropValue> {
        self.values.get(key)
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        match self.values.get(key)? {
            PropValue::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn get_u64(&self, key: &str) -> Option<u64> {
        match self.values.get(key)? {
            PropValue::U64(n) => Some(*n),
            _ => None,
        }
    }

    pub fn get_f64(&self, key: &str) -> Option<f64> {
        match self.values.get(key)? {
            PropValue::F64(n) => Some(*n),
            _ => None,
        }
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.values.get(key)? {
            PropValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn set(&mut self, key: impl Into<String>, value: PropValue) {
        self.values.insert(key.into(), value);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &PropValue)> {
        self.values.iter().map(|(k, v)| (k.as_str(), v))
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropValue {
    String(String),
    U64(u64),
    F64(f64),
    Bool(bool),
    List(Vec<String>),
}

impl From<&str> for PropValue {
    fn from(s: &str) -> Self {
        Self::String(s.to_owned())
    }
}

impl From<String> for PropValue {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<u64> for PropValue {
    fn from(n: u64) -> Self {
        Self::U64(n)
    }
}

impl From<usize> for PropValue {
    fn from(n: usize) -> Self {
        Self::U64(n as u64)
    }
}

impl From<f64> for PropValue {
    fn from(n: f64) -> Self {
        Self::F64(n)
    }
}

impl From<bool> for PropValue {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl From<Vec<String>> for PropValue {
    fn from(v: Vec<String>) -> Self {
        Self::List(v)
    }
}

impl std::fmt::Display for PropValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{s}"),
            Self::U64(n) => write!(f, "{n}"),
            Self::F64(n) => write!(f, "{n}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::List(v) => write!(f, "[{}]", v.join(", ")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_builder() {
        let node = Node::new(NodeKind::Program, "CLRG0100")
            .with_property("type", "batch".into())
            .with_property("loc", 1847u64.into())
            .with_property("complexity", 4.2f64.into());

        assert_eq!(node.kind, NodeKind::Program);
        assert_eq!(node.name, "CLRG0100");
        assert_eq!(node.properties.get_str("type"), Some("batch"));
        assert_eq!(node.properties.get_u64("loc"), Some(1847));
        assert_eq!(node.properties.get_f64("complexity"), Some(4.2));
    }

    #[test]
    fn node_with_program() {
        let node = Node::new(NodeKind::Paragraph, "1000-MAIN")
            .with_program("CLRG0100");

        assert_eq!(node.program, Some("CLRG0100".to_owned()));
    }

    #[test]
    fn properties_empty() {
        let props = Properties::default();
        assert!(props.is_empty());
        assert_eq!(props.get("foo"), None);
    }

    #[test]
    fn properties_type_mismatch_returns_none() {
        let mut props = Properties::default();
        props.set("count", PropValue::U64(42));
        assert_eq!(props.get_str("count"), None);
        assert_eq!(props.get_u64("count"), Some(42));
    }

    #[test]
    fn node_kind_display() {
        assert_eq!(NodeKind::Program.to_string(), "program");
        assert_eq!(NodeKind::Paragraph.to_string(), "paragraph");
        assert_eq!(NodeKind::Copybook.to_string(), "copybook");
    }

    #[test]
    fn prop_value_from_conversions() {
        let _: PropValue = "hello".into();
        let _: PropValue = String::from("world").into();
        let _: PropValue = 42u64.into();
        let _: PropValue = 42usize.into();
        let _: PropValue = 3.14f64.into();
        let _: PropValue = true.into();
        let _: PropValue = vec!["a".to_owned(), "b".to_owned()].into();
    }
}
