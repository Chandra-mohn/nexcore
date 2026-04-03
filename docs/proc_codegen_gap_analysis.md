# ProcDSL Codegen Gap Analysis

> Capability-oriented audit of ProcDSL grammar vs Java L1 codegen.
> Grammar file (`grammar/nexflow/ProcDSL.g4`) is the authoritative source.
> Last updated: 2026-04-02.

## Current State

The Java L1 proc codegen generates a **linear single-stream pipeline**:

```
KafkaSource -> transform -> transform -> KafkaSink
```

The grammar defines a **multi-stream DAG with stateful operators, temporal reasoning, and fault tolerance**.

---

## What Works Today

### Process Definition
- `process <name> ... end` -> `*Job.java` with `main()` + `buildPipeline()`
- `parallelism N` -> `env.setParallelism(N)`
- `checkpoint_interval` -> `env.enableCheckpointing(ms, EXACTLY_ONCE)`
- `KAFKA_BOOTSTRAP_SERVERS` from environment variable

### Sources
- `receive <name> from kafka "topic"` -> `KafkaSource<T>` with Avro deser
- `schema <name>` -> typed `SpecificRecord`
- `key <field>` -> `.keyBy()`
- Watermark strategy (hardcoded 5s bounded out-of-order)

### Sinks
- `emit <name> to kafka "topic"` -> `KafkaSink<T>` with Avro ser
- `schema <name>` -> typed
- `DeliveryGuarantee.AT_LEAST_ONCE`

### Operators (6 core types)
- `transform <in> using <xform> into <out>` -> `.map(new *Function())`
- `route using/when` -> `ProcessFunction` + `OutputTag` side outputs
- `window tumbling/sliding/session` -> Flink window assigners with key-by
- `join <left> with <right>` -> `intervalJoin` (inner) / `coGroup` (left/right/outer)
- `enrich using <name>` -> `AsyncDataStream.unorderedWait`
- `aggregate using <name>` -> reference to aggregate function

---

## Architectural Gaps

These are structural issues that block multiple features. They must be resolved
before individual capability gaps can be addressed.

### A1. Stream Registry

**Problem:** The codegen uses a single `current_stream: String` variable and processes
statements linearly. The grammar defines named streams (`receive X into raw_orders`),
and operators reference streams by name (`join raw_orders with raw_customers`).

**Impact:** Blocks multi-stream joins, merge, route fan-out with named targets,
multiple sinks, and any pipeline that isn't strictly linear.

**Example the codegen cannot handle:**
```
receive orders from source into raw_orders
    from kafka "orders" key order_id schema order_event

receive customers from source into raw_customers
    from kafka "customers" key customer_id schema customer_event

join raw_orders with raw_customers
    on customer_id within 30 seconds type inner

transform joined using enrich_order into enriched

route enriched
    when amount > 10000 to high_value_stream
    otherwise to standard_stream

emit high_value to high_value_sink
    to kafka "high-value-orders" schema order_event

emit standard to standard_sink
    to kafka "standard-orders" schema order_event
```

Two inputs, a join, a route producing two named streams, two sinks.
Current codegen loses the first stream when the second `receive` overwrites `current_stream`.

**Fix:** Replace `current_stream: String` with `HashMap<String, StreamRef>` that tracks
all named streams. Each operator looks up inputs and registers outputs by name.

### A2. Expression Compiler

**Problem:** DSL expressions appear in 10+ contexts: `filter`, `route when`, `validate_input`,
`set`, `let`, `if/then`, `on complete when`, marker conditions, inline transform bodies.
The codegen passes them as raw strings into Java code.

**Impact:** No filtering, no conditional routing, no validation, no field mutation,
no conditional logic, no marker conditions. Essentially, no data-dependent behavior.

**Grammar expression language includes:**
- Boolean: `and`, `or`, `not`
- Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Null: `is null`, `is not null`
- Set membership: `x in (a, b, c)`, `x not in [a, b, c]`
- Contains: `contains(field, "text")`
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Field paths: `record.nested.field`, `array[0].field`
- Function calls: `lookup()`, `now()`, `count()`, user-defined
- Ternary: `field ? value_if_true : value_if_false`
- String interpolation: `"Order ${order_id} processed"`
- Object/array literals: `{ key: value }`, `[a, b, c]`
- Duration arithmetic: `now() + 7 days`, `field.sla_hours hours`

**Fix:** Build a shared expression compiler that generates type-safe Java from DSL
expressions. The L3 `java/xform/expression.rs` translator is a starting point but needs
Avro getter generation, `in`/`not in`, null checks, ternary, string interpolation,
and duration arithmetic.

### A3. Operator Nesting

**Problem:** The grammar allows operators inside `branch`, `parallel`, `if/else`,
`on_success/on_failure`, and `phase` blocks. The codegen only handles flat sequential
statements.

**Impact:** Cannot generate branching sub-pipelines, parallel fan-out with per-branch
processing, conditional operator chains, error recovery sub-pipelines, or phased execution.

**Example:**
```
parallel order_enrichment timeout 30 seconds require_all true
    branch pricing
        transform orders using apply_pricing into priced
    end
    branch inventory
        call external inventory_service
            endpoint "/api/check"
            timeout 5 seconds
    end
end
```

**Fix:** Make operator generation recursive. The `write_operator` function should accept
a list of statements (not just one) and generate nested blocks.

### A4. Type Flow

**Problem:** The codegen doesn't track how types change through the pipeline. After
`transform X using normalize into Y`, the output type changes from the input schema
to the transform's output schema. But the codegen doesn't know what type `Y` is.

**Impact:** Downstream operators generate untyped code. Join output types are unknown.
Aggregate output types are unknown. Cannot generate correct Avro `SpecificRecord` casts.

**Fix:** Build a type propagation pass that resolves the schema type at each point
in the pipeline by consulting the transform/rules definitions.

### A5. AST Enrichment

**Problem:** Many grammar features are parsed into `raw: String` fields in the AST
rather than structured types:
- `StateBlock { raw: String }`
- `MetricsBlock { raw: String }`
- `ResilienceBlock { raw: String }`
- `ProcessStatement::Evaluate { raw: String }`
- `ProcessStatement::Branch { raw: String }`
- `ProcessStatement::Correlation { raw: String }`
- `ProcessStatement::Completion { raw: String }`

**Impact:** The codegen cannot generate code for any of these because there's no
structured data to work with. The parser would need to be extended to produce
typed AST nodes for each of these blocks.

**Fix:** Extend the proc parser builder to produce structured AST types.
This is a prerequisite for: state management, metrics, resilience/error handling,
evaluate integration, branching, event correlation, and completion events.

---

## Capability Gaps

### CG-1. Stream Filtering and Projection

**Grammar supports:**
```
receive orders from source into raw_orders
    from kafka "orders" key order_id schema order_event
    filter order.status != "cancelled" and order.amount > 0

    project order_id, customer_id, amount, currency, status
    // or: project except internal_flags, debug_metadata
```

Also standalone filtering:
```
validate_input
    require order.amount > 0 else "Amount must be positive"
    require order.currency is not null else "Currency required"
```

And deduplication (a form of stateful filtering):
```
deduplicate by order_id window 24 hours
    on_duplicate
        emit to duplicate_sink
    end
```

**Codegen generates:** Nothing. Every record passes through unfiltered.

**What's needed:**
- Receive filter: `.filter(record -> <compiled expression>)` after source
- Projection: schema transformation selecting/dropping fields
- Validate: `ProcessFunction` with valid main output + invalid side output
- Dedup: `KeyedProcessFunction` with `ValueState<Boolean>` + `StateTtlConfig`

**Why this matters:** Without filtering, you process 100% of input when you may need 10%.
Without dedup, exactly-once breaks. Without validation, bad records crash transforms.

### CG-2. Windowed Aggregation

**Grammar supports 8 aggregate functions in windows:**
```
window tumbling 5 minutes
    key by account_id
    aggregate
        count() as txn_count
        sum(amount) as total_amount
        avg(amount) as avg_amount
        min(amount) as min_amount
        max(amount) as max_amount
        collect(category) as categories
        first(timestamp) as first_seen
        last(timestamp) as last_seen
    end
    state running_totals
    allowed lateness 30 seconds
    late data to late_sink
```

**Codegen generates:** Window assignment is correct, but aggregation is `.reduce((a, b) -> b)` --
a placeholder that keeps only the last record. No aggregate functions, no late data routing.

**What's needed:**
- Generate `AggregateFunction<Input, Accumulator, Output>` with:
  - `createAccumulator()` initializing all fields
  - `add(value, acc)` updating each aggregate
  - `getResult(acc)` extracting final values
  - `merge(a, b)` for session window merging
- Late data: `.sideOutputLateData(tag)` + separate sink
- State reference: connect window output to named state

### CG-3. Stateful Processing

**Grammar supports:**
```
state
    uses account_state_definition

    local txn_counter keyed by account_id
        type counter
        ttl sliding 24 hours
        cleanup on_checkpoint

    local risk_flags keyed by account_id
        type map
        ttl absolute 7 days
        cleanup background

    buffer pending_txns keyed by account_id
        type priority by amount
        ttl 1 hour
```

State types: `counter`, `gauge`, `map`, `list`
Buffer types: `fifo`, `lifo`, `priority by <field>`
TTL: `sliding` (reset on access) or `absolute` (fixed expiry)
Cleanup: `on_checkpoint`, `on_access`, `background`

**Codegen generates:** Nothing. `StateBlock` is `raw: String` in the AST.

**What's needed:**
- `ValueState<Long>` for counters, `ValueState<BigDecimal>` for gauges
- `MapState<String, Object>` for maps
- `ListState<T>` for lists and buffers
- `StateTtlConfig` with sliding/absolute TTL
- Cleanup: `cleanupFullSnapshot()`, `cleanupIncrementally()`, `cleanupInRocksdbCompactFilter()`
- Generated `*Context.java` with typed accessors:
  `getCounter()`, `incrementCounter()`, `hasFlag()`, `addToBuffer()`, etc.

**Why this matters:** Without state, every record is processed in isolation. No running
totals, no fraud windows, no dedup, no correlation. State is what separates streaming
from batch.

### CG-4. Event Correlation

**Grammar supports two patterns:**

**Await** (wait for a matching event from another stream):
```
await payment_confirmation
    until confirmation_event arrives
        matching on order_id
    timeout 30 minutes
        emit to timeout_sink
```

**Hold** (accumulate events until a condition is met):
```
hold batch_records in accumulation_buffer
    keyed by batch_id
    complete when count >= 100
    timeout 5 minutes
        dead_letter dlq_sink
```

Completion conditions: `count >= N`, `marker received`, `using <completion_rule>`
Timeout actions: `emit to <sink>`, `dead_letter <dlq>`, `skip_action`

**Codegen generates:** Raw text comment only.

**What's needed:**
- **Await:** `KeyedProcessFunction` with `ValueState<PendingEvent>` + `TimerService`
  - First event -> store in state, register timer
  - Matching event -> emit joined result, clear state, cancel timer
  - Timer fires -> emit to timeout side output
- **Hold:** `KeyedProcessFunction` with `ListState<T>` accumulator
  - Each event -> add to list, check completion condition
  - If complete -> emit batch, clear state
  - Timer fires -> emit partial or dead-letter

These are fundamental patterns: saga orchestration, batch completion, event sequencing.

### CG-5. Error Handling and Fault Tolerance

**Grammar supports per-operator-type error handlers:**
```
on error
    transform_error
        retry 3 times delay 1 second backoff exponential max_delay 30 seconds
    lookup_error
        skip
    rule_error
        dead_letter dlq_sink
    transform failure
        retry 3 times then
            log_error("Transform failed after retries")
            emit to error_sink
        end
end

checkpoint every 60 seconds to s3_checkpoint

when slow strategy drop alert after 30 seconds
```

Error types: `transform_error`, `lookup_error`, `rule_error`, `correlation_error`
(also two-word: `transform failure`, `lookup failure`, etc.)

Error actions: `skip`, `retry N times`, `dead_letter <sink>`

Retry options: `delay <duration>`, `backoff exponential|linear`, `max_delay <duration>`

Retry exhaustion: `then` block with `log_error()`, `emit to <sink>`, `transition to "state"`

Backpressure: `strategy block|drop|sample <rate>`, `alert after <duration>`

**Codegen generates:** Checkpoint interval only (from execution block, not resilience block).

**What's needed:**
- Per-operator try/catch with configurable action
- Retry with exponential/linear backoff and max delay
- DLQ routing via side output with error context
- Circuit breaker: failure counting + open/half-open/closed states
- Backpressure: rate limiting or sampling under load
- `then` block: fallback actions when retries exhausted

**Why this matters:** Without error handling, one bad record crashes the pipeline.
Production Flink jobs MUST have DLQ routing and retry logic.

### CG-6. Conditional Logic

**Grammar supports:**
```
if order.amount > 10000 and order.risk_score > 0.7 then
    evaluate using fraud_rules
    emit flagged_orders to fraud_review_sink
elseif order.amount > 5000 then
    evaluate using standard_rules
else
    transform orders using auto_approve into approved
endif

set order.processing_fee = order.amount * 0.02
let discount = lookup(discount_table, order.customer_tier)
set order.final_amount = order.amount - discount + order.processing_fee
```

**Codegen generates:** Nothing.

**What's needed:**
- `if/elseif/else` -> `ProcessFunction` with branching logic
- `set` -> Avro record field mutation (`record.put("field", value)`)
- `let` -> local variable computation
- These compose with all other operators (transforms inside if blocks)

### CG-7. External Integration

**Grammar supports:**
```
call external payment_service
    endpoint "/api/v1/charge"
    timeout 5 seconds
    retry 3 times
    circuit_breaker
        failure_threshold 5
        reset_timeout 30 seconds

call ml_service "fraud_detection"
    features: { amount, merchant_category, time_of_day, customer_age }
    timeout 2 seconds

schedule reminder after routing_result.sla_hours hours
    action send_reminder
    repeat until order.status == "completed"
```

**Codegen generates:** Nothing.

**What's needed:**
- Async HTTP client (non-blocking, with timeout/retry/circuit breaker)
- ML model serving client (feature vector construction + prediction parsing)
- Timer-based scheduling with dynamic durations and repeat-until conditions
- All integrated with error handling (retry, DLQ, etc.)

### CG-8. Temporal Orchestration (Markers and Phases)

**Grammar supports:**
```
business_date from settlement_calendar
processing_date auto

markers
    eod_1: when transactions.drained or transactions.count >= 1000000
    eod_2: when after 23:59 and inventory_ready
    api_ready: when api.settlement.ready
end

phase before eod_1
    receive transactions ...
    transform ...
    on complete when all_batches_processed signal eod_1 to orchestrator
end

phase between eod_1 and eod_2
    aggregate using daily_summary
    emit ...
end
```

Marker conditions: compound AND/OR on `stream.drained`, `stream.count >= N`,
`after <time>`, `<signal> received`, `api.<service>.<status>`

Phase specs: `before <marker>`, `after <marker>`, `between <m1> and <m2>`, `anytime`

Signals: `signal <marker> to <target>` (standalone or on phase completion)

**Codegen generates:** Nothing.

**What's needed:**
- Marker state tracking (which markers have fired)
- Marker condition evaluation (compound boolean on stream metrics, time, external signals)
- Phase gating (only execute phase statements when conditions met)
- Signal emission to external orchestrator
- Business calendar integration (already in L0 runtime)
- This is essentially a **workflow engine** embedded in the streaming pipeline

### CG-9. Observability

**Grammar supports:**
```
metrics
    counter processed_records
    histogram processing_latency
    gauge queue_depth
    rate throughput window 1 minute
end

emit_audit_event "order_approved"
    actor system "approval_engine"
    payload: { order_id: order.id, amount: order.amount, decision: "approved" }
```

Also: `transition to "state"` for state machine transitions.

**Codegen generates:** Logger declaration only.

**What's needed:**
- Flink `MetricGroup` integration for counter/histogram/gauge
- Rate metrics: counter + windowed calculation
- Audit events: `GenericRecord` construction + side output to audit topic
- State machine transitions: update state + emit transition event

### CG-10. Advanced Source/Sink Connectors

**Grammar supports 7 connector types:**
- `kafka` with group, offset, isolation, headers, format, registry, timestamp bounds
- `mongodb` with index, upsert
- `redis` with channel
- `parquet` with partition_by, schema_path
- `csv` with delimiter, quote, escape, header, null_value
- `scheduler` for cron-triggered sources
- `state_store` for reading from managed state

**Sink features:**
- `broadcast` / `round_robin` fanout
- `persist to <target> async/sync` with batch size, flush interval
- `persist on error continue/fail/emit to <dlq>`
- `reason`, `preserve_state`, `include_error_context`, `template`, `channel`, `payload`

**Codegen generates:** Kafka only (basic config). All others are stubs.

### CG-11. Embedded SQL

**Grammar supports:**
```
sql ```
    SELECT account_id, SUM(amount) as total
    FROM transactions
    WHERE status = 'completed'
    GROUP BY account_id
    HAVING SUM(amount) > 10000
``` as HighValueAccounts
```

**Codegen generates:** Nothing.

**What's needed:** Flink SQL integration via `tableEnv.executeSql()` with result
conversion back to DataStream.

---

## MAJOR GAP: Spark / Batch Code Generation

This is not a feature gap within the Flink codegen -- it's an **entirely missing code
generation target**. The grammar explicitly defines batch processing capabilities that
map to Apache Spark, not Flink.

### What the Grammar Defines

**Execution mode:**
```
process daily_settlement
    mode batch
    ...
end

process near_realtime_aggregation
    mode micro_batch 30 seconds
    ...
end
```

Three modes: `stream` (Flink), `batch` (Spark), `micro_batch` (Spark Structured Streaming).

**Batch-oriented sources:**
```
receive historical_data from source into records
    from parquet "/data/lake/transactions/year=2025/"
        partition_by year, month
        schema_path "schemas/transaction.avsc"

receive csv_import from source into raw
    from csv "/data/imports/accounts.csv"
        delimiter ","
        quote "\""
        header true
        null_value ""
```

Parquet with partition pruning and external schema. CSV with full parsing options.
These are inherently batch sources -- Flink handles them but Spark is the natural fit.

**Bounded Kafka reads:**
```
receive events from source into bounded_stream
    from kafka "events"
        from timestamp "2025-01-01T00:00:00Z" to timestamp "2025-12-31T23:59:59Z"
```

Time-bounded Kafka consumption for historical replay/backfill -- a Spark batch pattern.

**SQL for analytics:**
```
sql ```
    SELECT
        account_id,
        date_trunc('month', txn_date) as month,
        SUM(amount) as total,
        COUNT(*) as txn_count,
        AVG(amount) as avg_amount
    FROM transactions
    WHERE status = 'completed'
    GROUP BY account_id, date_trunc('month', txn_date)
    HAVING SUM(amount) > 10000
``` as MonthlyAccountSummary
```

While Flink SQL exists, complex analytical SQL with GROUP BY, HAVING, window functions,
and CTEs is Spark SQL territory. Spark SQL is the production standard for batch analytics.

**MongoDB persistence:**
```
emit results to output
    to mongodb "analytics_db"
        persist to monthly_summaries async
            batch size 1000
            flush interval 30 seconds
```

Batch writes with configurable batch size -- more natural in Spark's `foreachBatch` than
Flink's streaming sinks.

### What Would Need to Be Generated

**For `mode batch` -> Spark Job:**

```java
public class DailySettlementJob {
    public static void main(String[] args) {
        SparkSession spark = SparkSession.builder()
            .appName("daily_settlement")
            .getOrCreate();

        // Source: Parquet with partition pruning
        Dataset<Row> transactions = spark.read()
            .parquet("/data/lake/transactions/")
            .filter(col("year").equalTo(2025));

        // Transform: reuse L3 transform logic (same business functions)
        Dataset<Row> enriched = transactions
            .map(new EnrichTransactionFunction(), encoder);

        // SQL: analytical aggregation
        enriched.createOrReplaceTempView("transactions");
        Dataset<Row> summary = spark.sql(
            "SELECT account_id, SUM(amount) as total ... "
        );

        // Sink: write to Parquet/MongoDB
        summary.write()
            .mode(SaveMode.Overwrite)
            .partitionBy("year", "month")
            .parquet("/data/lake/summaries/");

        spark.stop();
    }
}
```

**For `mode micro_batch` -> Spark Structured Streaming:**

```java
Dataset<Row> stream = spark.readStream()
    .format("kafka")
    .option("subscribe", "events")
    .load();

StreamingQuery query = stream
    .map(new TransformFunction(), encoder)
    .writeStream()
    .trigger(Trigger.ProcessingTime("30 seconds"))
    .format("parquet")
    .start();
```

### Scope of the Gap

| Aspect | Flink (Streaming) | Spark (Batch) | Status |
|--------|-------------------|---------------|--------|
| Grammar support | `mode stream` | `mode batch`, `mode micro_batch` | Grammar: YES |
| CLI target | `--target java` generates Flink | No Spark target exists | CLI: NO |
| Job class | `*Job.java` with `StreamExecutionEnvironment` | Need `*SparkJob.java` with `SparkSession` | Codegen: NO |
| Source connectors | `KafkaSource` | `spark.read().parquet()`, `.csv()`, `.format("kafka")` | Codegen: NO |
| Sink connectors | `KafkaSink` | `dataset.write().parquet()`, `.format("mongo")` | Codegen: NO |
| SQL integration | Flink SQL (limited) | Spark SQL (full analytical SQL) | Codegen: NO |
| Transform reuse | L3 `MapFunction` | L3 logic reusable as Spark `MapFunction` | Partial |
| Rules reuse | L4 `Table.evaluate()` | L4 logic reusable as-is (pure Java) | Partial |
| Dependencies | flink-core, flink-kafka | spark-core, spark-sql, spark-avro | None |

### Key Insight: L3/L4 Logic is Reusable

The transform functions (L3) and decision tables (L4) are **pure Java logic** that
doesn't depend on Flink. A Spark job can call `new NormalizeOrderFunction().map(record)`
or `new CreditScoringTable().evaluate(input)` directly. The business logic doesn't need
to be regenerated -- only the **job orchestration** (sources, sinks, SQL, DAG) changes.

This means Spark codegen is primarily:
1. `*SparkJob.java` class generation (parallel to `*Job.java` for Flink)
2. Source/sink connector generation for Spark APIs
3. SQL block integration via `spark.sql()`
4. CLI `--target spark` or detecting `mode batch` and switching generators

### Rust Batch Equivalent

For the Rust target, the batch equivalent would be:
- DataFusion (SQL engine, Arrow-native)
- Polars (DataFrame library, fast batch processing)
- Both are listed in the codegen strategy as candidates

This is a separate gap from the Rust streaming runtime (tokio + rdkafka).

---

## Implementation Roadmap

Ordered by dependency (later phases depend on earlier ones):

| Phase | Scope | Prerequisite |
|-------|-------|-------------|
| **A** | Multi-stream foundation (stream registry, merge, multiple receive/emit) | None |
| **B** | Expression compiler (DSL -> Java for all contexts) | None |
| **C** | Filtering and validation (filter, project, validate, deduplicate) | B |
| **D** | Windowed aggregation (8 aggregate functions, late data, state ref) | A |
| **E** | AST enrichment (structured types for state/metrics/resilience/etc.) | None |
| **F** | Stateful processing (counter/gauge/map/list, TTL, cleanup, buffers) | E |
| **G** | Error handling (per-type handlers, retry/backoff, DLQ, circuit breaker) | E, B |
| **H** | Event correlation (await, hold, timeout, completion) | A, F |
| **I** | Conditional logic (if/else, set/let, operator nesting) | B |
| **J** | External integration (HTTP calls, ML service, scheduling) | G |
| **K** | Temporal orchestration (markers, phases, signals, business calendar) | F, B |
| **L** | Observability (metrics, audit events, state transitions) | E |
| **M** | Connectors (MongoDB, Redis, Parquet, CSV, scheduler, state_store) | A |
| **N** | Embedded SQL (Flink SQL integration) | A |
| **O** | **Spark batch codegen** (SparkJob, Parquet/CSV sources, Spark SQL, batch sinks) | B, M |
| **P** | **Spark Structured Streaming** (micro_batch mode, readStream/writeStream) | O |

### Target Matrix After All Phases

| DSL | Java/Flink (stream) | Java/Spark (batch) | Rust |
|-----|---------------------|--------------------|------|
| `.schema` | .avsc + helpers | .avsc + helpers (shared) | serde structs |
| `.xform` | MapFunction | Spark MapFunction (reuse L3 logic) | pure functions |
| `.rules` | Decision tables | Decision tables (reuse L4 logic) | pure functions |
| `.proc` mode stream | Flink Job | -- | tokio+rdkafka (backlog) |
| `.proc` mode batch | -- | Spark Job | DataFusion/Polars (backlog) |
| `.proc` mode micro_batch | -- | Structured Streaming Job | -- |
| `.proc` sql blocks | Flink SQL | Spark SQL | DataFusion SQL (backlog) |
