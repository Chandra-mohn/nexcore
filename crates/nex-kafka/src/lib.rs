mod client;
pub mod decoder;
pub mod error;
pub mod filter;
pub mod models;
pub mod pii;

pub use client::KafkaClient;
pub use decoder::decode_message;
pub use error::KafkaError;
pub use filter::matches_filter;
pub use models::{
    ConsumeOptions, DecodedMessage, KafkaConfig, MessageFilter, PartitionOffsets, PiiConfig,
    PiiPatternDef, RawMessage, TopicInfo,
};
pub use pii::PiiMasker;
