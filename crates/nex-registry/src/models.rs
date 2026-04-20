use serde::{Deserialize, Serialize};

/// Connection configuration for a Confluent Schema Registry instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

/// Summary information about a schema subject.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectInfo {
    pub name: String,
    pub versions: Vec<i32>,
    pub latest_version: i32,
    pub schema_type: String,
}

/// Full schema version details as returned by the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaVersionInfo {
    pub subject: String,
    pub version: i32,
    pub id: i32,
    pub schema_type: String,
    pub schema: String,
    pub references: Vec<SchemaReference>,
}

/// A reference from one schema to another.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaReference {
    pub name: String,
    pub subject: String,
    pub version: i32,
}

/// Result of a compatibility check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityResult {
    pub is_compatible: bool,
    pub messages: Vec<String>,
}

// -- Wire types matching the Confluent Schema Registry REST API responses --

#[derive(Debug, Deserialize)]
pub(crate) struct WireSchema {
    pub subject: Option<String>,
    pub version: Option<i32>,
    pub id: Option<i32>,
    #[serde(rename = "schemaType", default = "default_avro")]
    pub schema_type: String,
    pub schema: Option<String>,
    #[serde(default)]
    pub references: Vec<SchemaReference>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct WireRegisterId {
    pub id: i32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct WireCompatibility {
    pub is_compatible: bool,
    #[serde(default)]
    pub messages: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct WireError {
    pub error_code: i32,
    pub message: String,
}

fn default_avro() -> String {
    "AVRO".to_string()
}
