use serde::{Deserialize, Serialize};

use super::node::Properties;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EdgeKind {
    Calls,
    Performs,
    Reads,
    Writes,
    Uses,
    Accesses,
    Contains,
}

impl EdgeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Calls => "calls",
            Self::Performs => "performs",
            Self::Reads => "reads",
            Self::Writes => "writes",
            Self::Uses => "uses",
            Self::Accesses => "accesses",
            Self::Contains => "contains",
        }
    }

    /// The forward traversal verb in NexQuery.
    pub fn forward_verb(&self) -> &'static str {
        match self {
            Self::Calls => "calling",
            Self::Performs => "performing",
            Self::Reads => "reading",
            Self::Writes => "writing",
            Self::Uses => "using",
            Self::Accesses => "accessing",
            Self::Contains => "containing",
        }
    }

    /// The reverse traversal verb in NexQuery.
    pub fn reverse_verb(&self) -> &'static str {
        match self {
            Self::Calls => "called-by",
            Self::Performs => "performed-by",
            Self::Reads => "read-by",
            Self::Writes => "written-by",
            Self::Uses => "used-by",
            Self::Accesses => "accessed-by",
            Self::Contains => "within",
        }
    }

    /// Parse a NexQuery traversal verb into (EdgeKind, is_forward).
    pub fn from_verb(verb: &str) -> Option<(Self, bool)> {
        match verb {
            "calling" => Some((Self::Calls, true)),
            "called-by" => Some((Self::Calls, false)),
            "performing" => Some((Self::Performs, true)),
            "performed-by" => Some((Self::Performs, false)),
            "reading" => Some((Self::Reads, true)),
            "read-by" => Some((Self::Reads, false)),
            "writing" => Some((Self::Writes, true)),
            "written-by" => Some((Self::Writes, false)),
            "using" => Some((Self::Uses, true)),
            "used-by" => Some((Self::Uses, false)),
            "accessing" => Some((Self::Accesses, true)),
            "accessed-by" => Some((Self::Accesses, false)),
            "containing" => Some((Self::Contains, true)),
            "within" => Some((Self::Contains, false)),
            _ => None,
        }
    }
}

impl std::fmt::Display for EdgeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub kind: EdgeKind,
    pub properties: Properties,
}

impl Edge {
    pub fn new(kind: EdgeKind) -> Self {
        Self {
            kind,
            properties: Properties::default(),
        }
    }

    pub fn with_property(mut self, key: impl Into<String>, value: super::node::PropValue) -> Self {
        self.properties.set(key, value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_kind_verbs_roundtrip() {
        let kinds = [
            EdgeKind::Calls,
            EdgeKind::Performs,
            EdgeKind::Reads,
            EdgeKind::Writes,
            EdgeKind::Uses,
            EdgeKind::Accesses,
            EdgeKind::Contains,
        ];

        for kind in &kinds {
            let fwd = kind.forward_verb();
            let rev = kind.reverse_verb();

            let (parsed_fwd, is_fwd) = EdgeKind::from_verb(fwd).unwrap();
            assert_eq!(parsed_fwd, *kind);
            assert!(is_fwd);

            let (parsed_rev, is_fwd) = EdgeKind::from_verb(rev).unwrap();
            assert_eq!(parsed_rev, *kind);
            assert!(!is_fwd);
        }
    }

    #[test]
    fn edge_kind_from_verb_unknown() {
        assert!(EdgeKind::from_verb("foobar").is_none());
    }

    #[test]
    fn edge_with_properties() {
        let edge = Edge::new(EdgeKind::Calls)
            .with_property("call_type", "static".into());

        assert_eq!(edge.kind, EdgeKind::Calls);
        assert_eq!(edge.properties.get_str("call_type"), Some("static"));
    }
}
