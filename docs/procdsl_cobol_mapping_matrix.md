# ProcDSL Grammar -- COBOL Mapping Matrix

Which grammar constructs map from COBOL source vs which are
Nexflow-specific features added during modernization.

Roughly 40% of the grammar maps from COBOL. The remaining 60% is
modern architecture (streaming, event-driven, observability) that
architects add as part of the modernization design.

---

## Constructs That Map from COBOL

These are populated by the DSL emitters from COBOL AST analysis.

### Process Structure

| DSL Construct | COBOL Source | Emitter |
|--------------|-------------|---------|
| `process name ... end` | PROGRAM-ID | E4 |
| `mode batch` | Always -- COBOL is batch | E4 |
| `import ../schema/...` | WORKING-STORAGE references | E4 |
| `import ../transform/...` | Section-based paragraph groups | E4 |
| `import ../rules/...` | Section-based decision paragraphs | E4 |

### Data Sources and Sinks

| DSL Construct | COBOL Source | Emitter |
|--------------|-------------|---------|
| `receive name from file "path"` | FD Sequential + OPEN INPUT | E4 |
| `receive name from csv "path"` | FD Line-Sequential + OPEN INPUT | E4 |
| `receive name from db "path"` | FD Indexed + OPEN INPUT | E4 |
| `emit name to file "path"` | FD Sequential + OPEN OUTPUT | E4 |
| `emit name to csv "path"` | FD Line-Sequential + OPEN OUTPUT | E4 |
| `emit name to db "path"` | FD Indexed + OPEN OUTPUT | E4 |
| `schema name` | FD record names / WORKING-STORAGE groups | E1, E4 |

### Processing Logic

| DSL Construct | COBOL Source | Emitter |
|--------------|-------------|---------|
| `transform in using name into out` | Paragraph with reads/writes | E4 |
| `evaluate using name` | Paragraph with decision logic (multi I/O) | E4 |
| `route using rule` | EVALUATE with PERFORM in each WHEN branch | E4 |
| `if/elseif/else/endif` | IF/ELSE IF chains | E4 |
| `loop ... end` | PERFORM UNTIL with condition | E4 |
| `parallel ... branch ... end` | Independent sections (no shared fields) | E4 |

### Schema (E1)

| DSL Construct | COBOL Source | Emitter |
|--------------|-------------|---------|
| `schema name ... end` | 01-level group or prefix-based entity | E1 |
| `fields ... end` | Elementary PIC fields | E1 |
| `identity ... end` | Key field heuristic (id, key, code, number) | E1 |
| `pattern master_data/event_log` | Field naming heuristic | E1 |
| `name object ... end` | 05/10 group hierarchy (non-elementary children) | E1 |
| `name list<object> ... end` | OCCURS on group items | E1 |
| `constraints ... enum() ... end` | Level-88 condition values | E1 |

### Transform (E2)

| DSL Construct | COBOL Source | Emitter |
|--------------|-------------|---------|
| `transform name ... end` | Single-write paragraph | E2 |
| `transform_block name ... end` | Multi-write paragraph | E2 |
| `pure : true/false` | Side-effect analysis (Display, Read, Write, Call, ExecSql) | E2 |
| `input/output : type` | PIC-based type from DataDivision | E2 |
| `apply ... end` | COMPUTE, MOVE, ADD, SUBTRACT, MULTIPLY, DIVIDE expressions | E2 |
| `mappings ... end` | Multi-field assignments | E2 |
| `compose sequential` | Orchestrator paragraph with PERFORMs | E2 |
| `compose conditional` | EVALUATE with PERFORM in each WHEN branch | E2 |
| `on_error ... end` | ON SIZE ERROR in arithmetic statements | E2 |
| `validate_input ... end` | PIC-based constraints (non-negative, max length) | E2 |

### Rules (E3)

| DSL Construct | COBOL Source | Emitter |
|--------------|-------------|---------|
| `decision_table name ... end` | EVALUATE (match) with 2+ arms | E3 |
| `hit_policy first_match` | COBOL EVALUATE is first-match | E3 |
| `given: ... decide: ... return:` | reads/writes attributes -> params/matrix/returns | E3 |
| `rule name: ... end` | IF/ELSE IF chain with 2+ branches | E3 |
| `if/elseif/else/endif` | Extracted conditions from COBOL IF predicates | E3 |
| `set field = value` | MOVE actions in IF/EVALUATE branches | E3 |

---

## Constructs NOT Mapped from COBOL

These are Nexflow-specific features. The emitters set them to None.
Architects add them during modernization design.

### Stream Processing (no COBOL equivalent)

| DSL Construct | Purpose | Why Not from COBOL |
|--------------|---------|-------------------|
| `mode stream` | Unbounded event processing | COBOL is batch -- finite input, deterministic order |
| `mode micro_batch` | Spark Structured Streaming | COBOL has no micro-batch concept |
| `watermark delay duration` | Event-time ordering tolerance | Batch input is fully ordered |
| `late data to sink` | Out-of-order event routing | Batch has no late arrivals |
| `allowed lateness duration` | How late events can be | Batch has no lateness |
| `window session gap duration` | Gap-based session grouping | Stream-specific temporal concept |
| `window sliding size every slide` | Overlapping time windows | COBOL processes each record once |
| `backpressure strategy drop/block` | Flow control under load | Batch reads at its own pace |

### Event-Driven Architecture (no COBOL equivalent)

| DSL Construct | Purpose | Why Not from COBOL |
|--------------|---------|-------------------|
| `await event until match timeout` | Wait for correlated event | COBOL is sequential, not event-driven |
| `hold records keyed by key complete when` | Accumulate until condition | Saga/batch-completion pattern |
| `on commit emit completion to sink` | Commit notification | Transaction lifecycle events |
| `on commit failure emit to sink` | Failure notification | Compensation events |
| `signal marker to target` | Inter-process signaling | COBOL has no pub/sub |
| `state_machine name` | Event-sourced state machine | COBOL has no formal state machine |
| `transition to "state"` | State machine transition | COBOL has no state transitions |
| `markers ... end` | EOD / stream drain detection | Real-time marker conditions |
| `phase before/after/between marker` | Marker-gated execution phases | Real-time orchestration |
| `on complete when expr signal marker` | Phase completion signals | Event-driven coordination |

### Connector Configuration (no COBOL equivalent)

| DSL Construct | Purpose | Why Not from COBOL |
|--------------|---------|-------------------|
| `from kafka "topic"` | Kafka consumer | COBOL has files, not topics |
| `offset latest/earliest` | Consumer group start position | Kafka-specific |
| `isolation read_committed` | Transaction isolation level | Kafka-specific |
| `headers: { key: value }` | Kafka message headers | Kafka-specific |
| `format json/avro/protobuf` | Serialization format | Kafka-specific |
| `registry "url"` | Schema registry | Confluent-specific |
| `from mongodb "collection"` | MongoDB source | COBOL has no MongoDB |
| `from redis channel: expr` | Redis pub/sub | COBOL has no Redis |
| `from scheduler` | Cron-triggered source | COBOL uses JCL for scheduling |
| `from state_store "name"` | Read from managed state | Stream-specific state |
| `broadcast / round_robin` | Sink distribution strategy | COBOL writes to one file |
| `persist to target async/sync` | Async database writes | COBOL I/O is synchronous |
| `compaction / retention` | Log compaction policy | Kafka-specific |
| `upsert by field` | Upsert writes | Database-specific |
| `template "pattern"` | Output templating | No COBOL equivalent |

### Observability (no COBOL equivalent)

| DSL Construct | Purpose | Why Not from COBOL |
|--------------|---------|-------------------|
| `metrics ... end` | counter/histogram/gauge/rate | COBOL has no metrics framework |
| `emit_audit_event "name"` | Structured audit trail | COBOL uses DISPLAY for logging |
| `actor system/user` | Audit actor identification | No actor model in COBOL |

### Resilience (no COBOL equivalent)

| DSL Construct | Purpose | Why Not from COBOL |
|--------------|---------|-------------------|
| `circuit_breaker threshold reset` | Failure circuit breaking | COBOL has no circuit breakers |
| `retry N times delay backoff` | Retry with exponential backoff | COBOL abends or continues |
| `dead_letter sink` | Failed record routing | COBOL has no DLQ pattern |
| `checkpoint every duration to target` | Stream state snapshots | COBOL has no checkpointing |

### External Integration (no COBOL equivalent)

| DSL Construct | Purpose | Why Not from COBOL |
|--------------|---------|-------------------|
| `call external endpoint timeout retry` | REST API with resilience | COBOL CALL is local subprogram |
| `call ml_service "model" features` | ML model inference | No ML in COBOL |
| `schedule action after duration repeat` | Timer-based deferred execution | COBOL is synchronous |

### Advanced Processing (no COBOL equivalent)

| DSL Construct | Purpose | Why Not from COBOL |
|--------------|---------|-------------------|
| `deduplicate by field window duration` | Stateful deduplication | COBOL assumes unique input |
| `add_flag "name"` | Runtime metadata tagging | No runtime metadata in COBOL |
| `add_metadata "key" = expr` | Dynamic metadata | No dynamic metadata in COBOL |
| `adjust_score by field` | Score accumulation | Scoring is application-specific |
| `interpolated string "${expr}"` | String interpolation | COBOL uses STRING verb |

---

## Summary

| Category | Constructs | Maps from COBOL |
|----------|-----------|----------------|
| Process structure | 5 | Yes |
| Data sources/sinks | 7 | Yes (via FD entries) |
| Processing logic | 6 | Yes (paragraphs, PERFORM, EVALUATE) |
| Schema (E1) | 7 | Yes (WORKING-STORAGE, PIC, level hierarchy) |
| Transform (E2) | 10 | Yes (expressions, purity, errors, validation) |
| Rules (E3) | 6 | Yes (EVALUATE, IF chains, MOVE actions) |
| Stream processing | 8 | No |
| Event-driven | 10 | No |
| Connector config | 15 | No |
| Observability | 3 | No |
| Resilience | 4 | No |
| External integration | 3 | No |
| Advanced processing | 5 | No |

**41 constructs map from COBOL (40%)**
**48 constructs are Nexflow-specific (60%)**

The emitters produce the 40%. Architects design the 60% during modernization.
