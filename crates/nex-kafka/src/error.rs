use thiserror::Error;

#[derive(Debug, Error)]
pub enum KafkaError {
    #[error("Kafka client error: {0}")]
    Client(String),

    #[error("Kafka consumer error: {0}")]
    Consumer(String),

    #[error("Kafka producer error: {0}")]
    Producer(String),

    #[error("Message decode error: {0}")]
    Decode(String),

    #[error("Invalid configuration: {0}")]
    Config(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Schema Registry error: {0}")]
    Registry(#[from] nex_registry::RegistryError),

    #[error("Timeout: {0}")]
    Timeout(String),
}

impl From<rdkafka::error::KafkaError> for KafkaError {
    fn from(e: rdkafka::error::KafkaError) -> Self {
        KafkaError::Client(e.to_string())
    }
}
