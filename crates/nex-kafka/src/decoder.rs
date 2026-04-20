use crate::models::{DecodedMessage, RawMessage};

/// Decode a raw Kafka message into a frontend-friendly format.
///
/// Detection order:
/// 1. JSON -- first byte is `{` (0x7B) or `[` (0x5B)
/// 2. Confluent Avro wire format -- magic byte 0x00 + 4-byte schema ID
///    (requires schema registry; deferred to async caller)
/// 3. Fallback -- hex-encode raw bytes
pub fn decode_message(raw: &RawMessage) -> DecodedMessage {
    let (value, format) = match &raw.value {
        Some(bytes) if bytes.is_empty() => (String::new(), "empty"),
        Some(bytes) => decode_value(bytes),
        None => ("null".to_string(), "null"),
    };

    let key = raw.key.as_ref().map(|k| {
        // Try UTF-8 first, fall back to hex
        String::from_utf8(k.clone()).unwrap_or_else(|_| hex_encode(k))
    });

    let headers = raw
        .headers
        .iter()
        .map(|(k, v)| {
            let val = String::from_utf8(v.clone()).unwrap_or_else(|_| hex_encode(v));
            (k.clone(), val)
        })
        .collect();

    let size = raw.value.as_ref().map_or(0, Vec::len)
        + raw.key.as_ref().map_or(0, Vec::len);

    DecodedMessage {
        offset: raw.offset,
        partition: raw.partition,
        timestamp: raw.timestamp,
        key,
        value,
        headers,
        value_format: format.to_string(),
        size_bytes: size,
    }
}

fn decode_value(bytes: &[u8]) -> (String, &'static str) {
    if bytes.is_empty() {
        return (String::new(), "empty");
    }

    // Check for JSON
    let first = bytes[0];
    if first == b'{' || first == b'[' {
        if let Ok(s) = std::str::from_utf8(bytes) {
            // Validate it's actually JSON
            if serde_json::from_str::<serde_json::Value>(s).is_ok() {
                return (s.to_string(), "json");
            }
        }
    }

    // Check for Confluent Avro wire format (magic byte 0x00 + 4-byte schema ID)
    if bytes.len() > 5 && bytes[0] == 0x00 {
        let schema_id = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
        // We can't decode Avro synchronously without the registry.
        // Return a marker that the caller can use to trigger async decode.
        return (
            format!("{{\"_confluent_avro\": true, \"schema_id\": {schema_id}, \"raw_hex\": \"{}\"}}", hex_encode(&bytes[5..])),
            "avro",
        );
    }

    // Try plain UTF-8 string
    if let Ok(s) = std::str::from_utf8(bytes) {
        if s.chars().all(|c| !c.is_control() || c == '\n' || c == '\r' || c == '\t') {
            return (s.to_string(), "text");
        }
    }

    // Fallback: hex
    (hex_encode(bytes), "raw")
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<Vec<_>>()
        .join(" ")
}
