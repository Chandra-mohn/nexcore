use std::time::Duration;

use rdkafka::admin::AdminClient;
use rdkafka::client::DefaultClientContext;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::message::{Headers, Message};
use rdkafka::producer::{BaseProducer, BaseRecord, Producer};
use rdkafka::topic_partition_list::Offset;
use rdkafka::TopicPartitionList;

use crate::error::KafkaError;
use crate::models::{ConsumeOptions, KafkaConfig, PartitionOffsets, RawMessage, TopicInfo};

/// Kafka client for topic inspection, message consumption, and production.
///
/// All operations are synchronous (librdkafka C library). Callers should
/// run these on a blocking thread pool.
#[derive(Debug)]
pub struct KafkaClient {
    config: KafkaConfig,
}

impl KafkaClient {
    pub fn new(config: &KafkaConfig) -> Result<Self, KafkaError> {
        if config.bootstrap_servers.is_empty() {
            return Err(KafkaError::Config(
                "Bootstrap servers must not be empty".into(),
            ));
        }
        Ok(Self {
            config: config.clone(),
        })
    }

    fn base_config(&self) -> ClientConfig {
        let mut cfg = ClientConfig::new();
        cfg.set("bootstrap.servers", &self.config.bootstrap_servers);
        cfg.set("security.protocol", &self.config.security_protocol);

        if let Some(mechanism) = &self.config.sasl_mechanism {
            cfg.set("sasl.mechanism", mechanism);
        }
        if let Some(username) = &self.config.sasl_username {
            cfg.set("sasl.username", username);
        }
        if let Some(password) = &self.config.sasl_password {
            cfg.set("sasl.password", password);
        }

        cfg
    }

    /// Verify connectivity by fetching cluster metadata.
    pub fn test_connection(&self) -> Result<String, KafkaError> {
        let admin: AdminClient<DefaultClientContext> =
            self.base_config().create()?;
        let metadata = admin
            .inner()
            .fetch_metadata(None, Duration::from_secs(10))
            .map_err(|e| KafkaError::Client(e.to_string()))?;

        let broker_count = metadata.brokers().len();
        let topic_count = metadata.topics().len();
        Ok(format!(
            "Connected -- {broker_count} broker(s), {topic_count} topic(s)"
        ))
    }

    /// List all topics with partition counts and estimated message counts.
    pub fn list_topics(&self) -> Result<Vec<TopicInfo>, KafkaError> {
        let consumer: BaseConsumer = self
            .base_config()
            .set("group.id", "nexstudio-browser")
            .create()?;

        let metadata = consumer
            .fetch_metadata(None, Duration::from_secs(10))
            .map_err(|e| KafkaError::Client(e.to_string()))?;

        let mut topics = Vec::new();

        for topic in metadata.topics() {
            let name = topic.name().to_string();

            // Skip internal topics
            if name.starts_with("__") || name.starts_with("_schemas") {
                continue;
            }

            let partitions = topic.partitions().len() as i32;

            // Estimate message count from watermark offsets
            let mut total_msgs: i64 = 0;
            let mut replication = 0i32;

            for p in topic.partitions() {
                replication = p.replicas().len() as i32;
                if let Ok((low, high)) = consumer.fetch_watermarks(
                    &name,
                    p.id(),
                    Duration::from_secs(5),
                ) {
                    total_msgs += high - low;
                }
            }

            topics.push(TopicInfo {
                name,
                partitions,
                replication_factor: replication,
                message_count_estimate: Some(total_msgs),
            });
        }

        topics.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(topics)
    }

    /// Get watermark offsets for all partitions of a topic.
    pub fn get_topic_offsets(&self, topic: &str) -> Result<Vec<PartitionOffsets>, KafkaError> {
        let consumer: BaseConsumer = self
            .base_config()
            .set("group.id", "nexstudio-browser")
            .create()?;

        let metadata = consumer
            .fetch_metadata(Some(topic), Duration::from_secs(10))
            .map_err(|e| KafkaError::Client(e.to_string()))?;

        let topic_meta = metadata
            .topics()
            .first()
            .ok_or_else(|| KafkaError::Client(format!("Topic '{topic}' not found")))?;

        let mut offsets = Vec::new();
        for p in topic_meta.partitions() {
            let (low, high) = consumer
                .fetch_watermarks(topic, p.id(), Duration::from_secs(5))
                .map_err(|e| KafkaError::Client(e.to_string()))?;
            offsets.push(PartitionOffsets {
                partition: p.id(),
                low,
                high,
            });
        }
        Ok(offsets)
    }

    /// Consume raw messages from a topic.
    pub fn consume(&self, options: &ConsumeOptions) -> Result<Vec<RawMessage>, KafkaError> {
        let consumer: BaseConsumer = self
            .base_config()
            .set("group.id", "nexstudio-peek")
            .set("auto.offset.reset", "latest")
            .set("enable.auto.commit", "false")
            .create()?;

        // Determine partitions and offsets
        let metadata = consumer
            .fetch_metadata(Some(&options.topic), Duration::from_secs(10))
            .map_err(|e| KafkaError::Client(e.to_string()))?;

        let topic_meta = metadata.topics().first().ok_or_else(|| {
            KafkaError::Client(format!("Topic '{}' not found", options.topic))
        })?;

        let mut tpl = TopicPartitionList::new();

        let partitions: Vec<i32> = if let Some(p) = options.partition {
            vec![p]
        } else {
            topic_meta.partitions().iter().map(|p| p.id()).collect()
        };

        for p in &partitions {
            let offset = match options.offset.as_str() {
                "earliest" => Offset::Beginning,
                "latest" => {
                    // For "latest", go back `limit` messages from the end
                    let (_, high) = consumer
                        .fetch_watermarks(&options.topic, *p, Duration::from_secs(5))
                        .map_err(|e| KafkaError::Client(e.to_string()))?;
                    let start = (high - options.limit as i64).max(0);
                    Offset::Offset(start)
                }
                s => {
                    let n: i64 = s
                        .parse()
                        .map_err(|_| KafkaError::Config(format!("Invalid offset: {s}")))?;
                    Offset::Offset(n)
                }
            };
            tpl.add_partition_offset(&options.topic, *p, offset)
                .map_err(|e| KafkaError::Client(e.to_string()))?;
        }

        consumer
            .assign(&tpl)
            .map_err(|e| KafkaError::Consumer(e.to_string()))?;

        let timeout = Duration::from_millis(options.timeout_ms);
        let mut messages = Vec::new();

        // Poll until we have enough messages or timeout
        let deadline = std::time::Instant::now() + timeout;

        while messages.len() < options.limit && std::time::Instant::now() < deadline {
            match consumer.poll(Duration::from_millis(500)) {
                Some(Ok(msg)) => {
                    let raw = RawMessage {
                        offset: msg.offset(),
                        partition: msg.partition(),
                        timestamp: msg.timestamp().to_millis(),
                        key: msg.key().map(|k| k.to_vec()),
                        value: msg.payload().map(|v| v.to_vec()),
                        headers: msg
                            .headers()
                            .map(|hdrs| {
                                (0..hdrs.count())
                                    .filter_map(|i| {
                                        hdrs.get_as::<[u8]>(i).ok().map(|h| {
                                            (h.key.to_string(), h.value.map_or_else(Vec::new, <[u8]>::to_vec))
                                        })
                                    })
                                    .collect()
                            })
                            .unwrap_or_default(),
                    };
                    messages.push(raw);
                }
                Some(Err(e)) => {
                    return Err(KafkaError::Consumer(e.to_string()));
                }
                None => {
                    // No message within poll interval, continue
                }
            }
        }

        Ok(messages)
    }

    /// Produce a single message to a topic. Returns the partition it was sent to.
    pub fn produce(
        &self,
        topic: &str,
        key: Option<&str>,
        value: &[u8],
        headers: Option<&[(String, String)]>,
    ) -> Result<(), KafkaError> {
        let producer: BaseProducer = self.base_config().create()?;

        let mut record = BaseRecord::to(topic).payload(value);
        if let Some(k) = key {
            record = record.key(k);
        }

        // rdkafka headers require owned OwnedHeaders; for simplicity we skip
        // header support in initial implementation
        let _ = headers;

        producer
            .send(record)
            .map_err(|(e, _)| KafkaError::Producer(e.to_string()))?;

        producer.flush(Duration::from_secs(5)).map_err(|e| {
            KafkaError::Producer(format!("Flush failed: {e}"))
        })?;

        Ok(())
    }
}
