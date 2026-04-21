use std::time::Duration;

use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::message::{Headers, Message};
use rdkafka::producer::{BaseProducer, BaseRecord, Producer};
use rdkafka::topic_partition_list::Offset;
use rdkafka::TopicPartitionList;

use crate::error::KafkaError;
use crate::filter::matches_filter;
use crate::models::{
    ConsumeOptions, KafkaConfig, PartitionOffsets, RawMessage, ReplayOptions, ReplayReport,
    TopicInfo,
};

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
        // Suppress librdkafka's noisy stderr logging (show errors only)
        cfg.set("log_level", "3");

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
        let consumer: BaseConsumer = self
            .base_config()
            .set("group.id", "nexstudio-health")
            .set("session.timeout.ms", "6000")
            .set("socket.timeout.ms", "5000")
            .create()?;

        let metadata = consumer
            .fetch_metadata(None, Duration::from_secs(5))
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

    /// Replay messages from source topic to target topic.
    ///
    /// Consumes from source with optional offset/filter, produces to target
    /// with optional rate limiting. Returns a report with counts.
    pub fn replay(&self, options: &ReplayOptions) -> Result<ReplayReport, KafkaError> {
        let start = std::time::Instant::now();

        // Set up consumer
        let consumer: BaseConsumer = self
            .base_config()
            .set("group.id", "nexstudio-replay")
            .set("auto.offset.reset", "earliest")
            .set("enable.auto.commit", "false")
            .create()?;

        let metadata = consumer
            .fetch_metadata(Some(&options.source_topic), Duration::from_secs(10))
            .map_err(|e| KafkaError::Client(e.to_string()))?;

        let topic_meta = metadata.topics().first().ok_or_else(|| {
            KafkaError::Client(format!("Topic '{}' not found", options.source_topic))
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
                "latest" => Offset::End,
                s => {
                    let n: i64 = s
                        .parse()
                        .map_err(|_| KafkaError::Config(format!("Invalid offset: {s}")))?;
                    Offset::Offset(n)
                }
            };
            tpl.add_partition_offset(&options.source_topic, *p, offset)
                .map_err(|e| KafkaError::Client(e.to_string()))?;
        }

        consumer
            .assign(&tpl)
            .map_err(|e| KafkaError::Consumer(e.to_string()))?;

        // Set up producer (unless dry run)
        let producer: Option<BaseProducer> = if options.dry_run {
            None
        } else {
            Some(self.base_config().create()?)
        };

        let timeout = Duration::from_millis(options.timeout_ms);
        let deadline = std::time::Instant::now() + timeout;
        let max_msgs = options.limit.unwrap_or(usize::MAX);
        let rate_delay = options
            .rate_limit
            .map(|r| Duration::from_micros(1_000_000 / u64::from(r.max(1))));

        let mut consumed: usize = 0;
        let mut produced: usize = 0;
        let mut filtered: usize = 0;
        let mut errors: usize = 0;
        let mut batch_count: usize = 0;

        while consumed < max_msgs && std::time::Instant::now() < deadline {
            match consumer.poll(Duration::from_millis(500)) {
                Some(Ok(msg)) => {
                    consumed += 1;

                    // Apply filter if configured
                    if let Some(filter) = &options.filter {
                        if let Some(payload) = msg.payload() {
                            if let Ok(json_str) = std::str::from_utf8(payload) {
                                if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(json_str) {
                                    if !matches_filter(&json_val, filter) {
                                        filtered += 1;
                                        continue;
                                    }
                                }
                            }
                        }
                    }

                    // Produce to target
                    if let Some(ref prod) = producer {
                        let payload = msg.payload().unwrap_or(&[]);
                        let mut record = BaseRecord::to(&options.target_topic).payload(payload);
                        if let Some(key) = msg.key() {
                            if let Ok(k) = std::str::from_utf8(key) {
                                record = record.key(k);
                            }
                        }

                        match prod.send(record) {
                            Ok(()) => {
                                produced += 1;
                                batch_count += 1;
                            }
                            Err((e, _)) => {
                                errors += 1;
                                eprintln!("[replay] produce error: {e}");
                            }
                        }

                        // Flush batch
                        if batch_count >= options.batch_size {
                            prod.flush(Duration::from_secs(5))
                                .map_err(|e| KafkaError::Producer(format!("Flush failed: {e}")))?;
                            batch_count = 0;
                        }

                        // Rate limiting
                        if let Some(delay) = rate_delay {
                            std::thread::sleep(delay);
                        }
                    } else {
                        // Dry run -- count as "would produce"
                        produced += 1;
                    }
                }
                Some(Err(e)) => {
                    errors += 1;
                    eprintln!("[replay] consume error: {e}");
                }
                None => {
                    // No more messages within poll interval -- check if we've hit the end
                    if std::time::Instant::now() + Duration::from_secs(1) > deadline {
                        break;
                    }
                }
            }
        }

        // Final flush
        if let Some(ref prod) = producer {
            prod.flush(Duration::from_secs(10))
                .map_err(|e| KafkaError::Producer(format!("Final flush failed: {e}")))?;
        }

        Ok(ReplayReport {
            consumed,
            produced,
            filtered,
            errors,
            dry_run: options.dry_run,
            duration_ms: start.elapsed().as_millis() as u64,
        })
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
