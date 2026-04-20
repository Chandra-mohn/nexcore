use serde::{Deserialize, Serialize};

use crate::error::RegistryError;

/// Supported serialization formats for schema registration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SerializationFormat {
    Json,
    Avro,
    ConfluentAvro,
    Protobuf,
}

impl SerializationFormat {
    /// Convert to the schema type string used by the Confluent Schema Registry API.
    pub fn registry_schema_type(&self) -> &'static str {
        match self {
            Self::Json => "JSON",
            Self::Avro | Self::ConfluentAvro => "AVRO",
            Self::Protobuf => "PROTOBUF",
        }
    }

    pub fn from_str_loose(s: &str) -> Option<Self> {
        match s.to_lowercase().trim() {
            "json" => Some(Self::Json),
            "avro" => Some(Self::Avro),
            "confluent_avro" => Some(Self::ConfluentAvro),
            "protobuf" => Some(Self::Protobuf),
            _ => None,
        }
    }
}

/// Subject naming strategy for schema registration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubjectNamingStrategy {
    TopicNameStrategy,
    RecordNameStrategy,
    TopicRecordNameStrategy,
}

impl SubjectNamingStrategy {
    /// Derive a subject name from schema name and optional topic.
    pub fn derive_subject(&self, schema_name: &str, topic: Option<&str>) -> String {
        match self {
            Self::TopicNameStrategy => {
                // topic-value (or schema-name-value if no topic)
                let base = topic.unwrap_or(schema_name);
                format!("{base}-value")
            }
            Self::RecordNameStrategy => {
                // schema name as-is
                schema_name.to_string()
            }
            Self::TopicRecordNameStrategy => {
                // topic-schema_name
                match topic {
                    Some(t) => format!("{t}-{schema_name}"),
                    None => schema_name.to_string(),
                }
            }
        }
    }
}

/// Profile-level serialization configuration.
/// Stored per registry profile in the project config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSerializationConfig {
    pub default_format: SerializationFormat,
    pub allowed_formats: Vec<SerializationFormat>,
    pub subject_naming: SubjectNamingStrategy,
}

impl Default for ProfileSerializationConfig {
    fn default() -> Self {
        Self {
            default_format: SerializationFormat::Json,
            allowed_formats: vec![
                SerializationFormat::Json,
                SerializationFormat::Avro,
                SerializationFormat::ConfluentAvro,
            ],
            subject_naming: SubjectNamingStrategy::TopicNameStrategy,
        }
    }
}

/// Schema-level serialization declaration (from the .schema file's serialization block).
#[derive(Debug, Clone, Default)]
pub struct SchemaSerializationDecl {
    pub format: Option<SerializationFormat>,
    pub subject: Option<String>,
    pub registry: Option<String>,
}

/// Resolved serialization config -- the final answer after hierarchy resolution.
#[derive(Debug, Clone, Serialize)]
pub struct ResolvedSerialization {
    pub format: SerializationFormat,
    pub schema_type: String,
    pub subject: String,
    pub subject_naming: SubjectNamingStrategy,
}

/// Resolve the effective serialization configuration.
///
/// Resolution hierarchy (highest priority first):
/// 1. Schema-level declaration (from `.schema` file `serialization` block)
/// 2. Profile-level config (from project settings per registry profile)
/// 3. Built-in default (json)
///
/// The resolved format is validated against the profile's allowed formats.
pub fn resolve_serialization(
    schema_name: &str,
    schema_decl: &SchemaSerializationDecl,
    profile_config: &ProfileSerializationConfig,
    topic: Option<&str>,
) -> Result<ResolvedSerialization, RegistryError> {
    // Determine format: schema-level > profile default > json
    let requested_format = schema_decl
        .format
        .unwrap_or(profile_config.default_format);

    // Validate against allowed formats
    let format = if profile_config.allowed_formats.contains(&requested_format) {
        requested_format
    } else {
        // Fall back to profile default if requested format is not allowed
        if profile_config.allowed_formats.contains(&profile_config.default_format) {
            profile_config.default_format
        } else {
            // Last resort: first allowed format
            *profile_config
                .allowed_formats
                .first()
                .unwrap_or(&SerializationFormat::Json)
        }
    };

    // Determine subject: schema-level > derived from naming strategy
    let subject = schema_decl
        .subject
        .clone()
        .unwrap_or_else(|| {
            profile_config
                .subject_naming
                .derive_subject(schema_name, topic)
        });

    Ok(ResolvedSerialization {
        schema_type: format.registry_schema_type().to_string(),
        format,
        subject,
        subject_naming: profile_config.subject_naming,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_format_overrides_profile() {
        let profile = ProfileSerializationConfig::default();
        let schema = SchemaSerializationDecl {
            format: Some(SerializationFormat::Avro),
            subject: None,
            registry: None,
        };

        let resolved = resolve_serialization("CustomerEvent", &schema, &profile, None).unwrap();
        assert_eq!(resolved.format, SerializationFormat::Avro);
        assert_eq!(resolved.schema_type, "AVRO");
    }

    #[test]
    fn falls_back_to_profile_default() {
        let profile = ProfileSerializationConfig {
            default_format: SerializationFormat::ConfluentAvro,
            ..Default::default()
        };
        let schema = SchemaSerializationDecl::default();

        let resolved = resolve_serialization("Order", &schema, &profile, None).unwrap();
        assert_eq!(resolved.format, SerializationFormat::ConfluentAvro);
    }

    #[test]
    fn disallowed_format_falls_back() {
        let profile = ProfileSerializationConfig {
            default_format: SerializationFormat::Json,
            allowed_formats: vec![SerializationFormat::Json],
            ..Default::default()
        };
        let schema = SchemaSerializationDecl {
            format: Some(SerializationFormat::Protobuf),
            subject: None,
            registry: None,
        };

        let resolved = resolve_serialization("Event", &schema, &profile, None).unwrap();
        assert_eq!(resolved.format, SerializationFormat::Json);
    }

    #[test]
    fn schema_subject_overrides_derived() {
        let profile = ProfileSerializationConfig::default();
        let schema = SchemaSerializationDecl {
            format: None,
            subject: Some("my-custom-subject".into()),
            registry: None,
        };

        let resolved = resolve_serialization("Order", &schema, &profile, None).unwrap();
        assert_eq!(resolved.subject, "my-custom-subject");
    }

    #[test]
    fn topic_name_strategy_derives_subject() {
        let profile = ProfileSerializationConfig::default();
        let schema = SchemaSerializationDecl::default();

        let resolved =
            resolve_serialization("Order", &schema, &profile, Some("orders")).unwrap();
        assert_eq!(resolved.subject, "orders-value");
    }

    #[test]
    fn record_name_strategy() {
        let profile = ProfileSerializationConfig {
            subject_naming: SubjectNamingStrategy::RecordNameStrategy,
            ..Default::default()
        };
        let schema = SchemaSerializationDecl::default();

        let resolved = resolve_serialization("CustomerEvent", &schema, &profile, None).unwrap();
        assert_eq!(resolved.subject, "CustomerEvent");
    }
}
