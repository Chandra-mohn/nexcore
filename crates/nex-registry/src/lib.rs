mod client;
pub mod error;
pub mod models;
pub mod resolver;

pub use client::RegistryClient;
pub use error::RegistryError;
pub use models::{
    CompatibilityResult, RegistryConfig, SchemaReference, SchemaVersionInfo, SubjectInfo,
};
pub use resolver::{
    ProfileSerializationConfig, ResolvedSerialization, SchemaSerializationDecl,
    SerializationFormat, SubjectNamingStrategy, resolve_serialization,
};
