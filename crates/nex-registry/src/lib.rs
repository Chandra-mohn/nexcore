mod client;
pub mod error;
pub mod models;

pub use client::RegistryClient;
pub use error::RegistryError;
pub use models::{
    CompatibilityResult, RegistryConfig, SchemaReference, SchemaVersionInfo, SubjectInfo,
};
