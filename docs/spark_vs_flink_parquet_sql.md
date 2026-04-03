# Spark vs Flink: SQL Querying from Parquet Files

> Decision context: Nexflow ProcDSL `mode batch` target selection.
> Conclusion: Spark is the right target for batch SQL on Parquet.

## Comparison Grid

| Aspect | Apache Spark | Apache Flink |
|---|---|---|
| **Parquet as SQL source** | First-class. `spark.read.parquet(path)` returns DataFrame, `createTempView` makes it SQL-queryable. This is Spark's core design center -- every optimization targets this path. | Secondary. Requires `TableEnvironment` with explicit DDL: `CREATE TABLE ... WITH ('connector'='filesystem', 'format'='parquet')`. Works but you feel the config overhead. Design center is streaming, not file scanning. |
| **Partition discovery** | Automatic. Reads Hive-style partitions (`year=2025/month=03/`) without configuration. Discovers partition columns from directory structure. Partition values become columns automatically. | Manual. Must declare partition columns in table DDL. No automatic discovery from directory layout. Misconfigure and you get full table scans. |
| **Partition pruning** | Automatic predicate pushdown into partition structure. `WHERE year = 2025 AND month = 3` skips all other directories at filesystem level before reading any Parquet data. Zero config, zero code. | Works when partitions declared correctly in DDL, but less transparent. Requires explicit partition column definition. Not as aggressive -- may read more data than necessary if DDL doesn't match physical layout. |
| **Predicate pushdown into Parquet internals** | Pushes filter predicates into Parquet row group statistics (min/max in footer). Skips entire row groups that can't match. Also pushes into page-level bloom filters (Parquet 1.12+). Columnar pruning reads only referenced columns. | Supports row group statistics pushdown. Less aggressive on page-level filters. Column pruning works. Overall fewer optimizations in the read path -- Flink's Parquet reader is functional but not as tuned. |
| **Window functions** | Complete: `ROW_NUMBER()`, `RANK()`, `DENSE_RANK()`, `LAG()`, `LEAD()`, `FIRST_VALUE()`, `LAST_VALUE()`, `NTH_VALUE()`, `NTILE()`, `PERCENT_RANK()`, `CUME_DIST()`. All battle-tested at petabyte scale. | Basics work: `ROW_NUMBER()`, `RANK()`, `LAG()`, `LEAD()`. Fewer advanced window functions. Designed for streaming windows first, analytical windows second. |
| **Grouping extensions** | `GROUP BY`, `CUBE`, `ROLLUP`, `GROUPING SETS`. Full dimensional analysis in a single query. | `GROUP BY` only. No `CUBE`, no `ROLLUP`, no `GROUPING SETS`. Multiple queries needed for dimensional analysis. |
| **Subqueries** | Correlated and uncorrelated subqueries. Scalar subqueries, `EXISTS`, `IN` with subquery. All optimized. | Limited correlated subquery support. Uncorrelated works. Some edge cases produce errors or poor plans. |
| **CTEs** | `WITH` clauses, recursive CTEs. Mature and widely used. | `WITH` clauses supported but less mature. Recursive CTEs limited. |
| **PIVOT / UNPIVOT** | Native `PIVOT` and `UNPIVOT` syntax. Cross-tabulation in SQL. | Not supported. Must use manual CASE/WHEN expressions. |
| **TABLESAMPLE** | Native `TABLESAMPLE (N PERCENT)` or `TABLESAMPLE (N ROWS)`. Useful for development and statistical analysis. | Not supported for batch. |
| **Lateral views / EXPLODE** | `LATERAL VIEW EXPLODE(array_col)` flattens arrays into rows. `POSEXPLODE` includes position. `INLINE` for arrays of structs. | `UNNEST` instead of `EXPLODE`. Works but less intuitive. Fewer built-in functions for nested data manipulation. |
| **UDAF** | User-defined aggregate functions via `Aggregator` class or pandas UDF. Type-safe with schema inference. Immediately usable in SQL. | UDAF via `AggregateFunction` class with `accumulate()`/`retract()`/`merge()`. More boilerplate. Works but heavier implementation burden. |
| **Query optimizer** | Catalyst: rule-based + cost-based optimization. Adaptive Query Execution (AQE) re-optimizes at runtime based on actual data statistics -- adjusts shuffle partitions, converts sort-merge to broadcast joins mid-execution, optimizes skewed joins. Years ahead of alternatives for batch. | Rule-based optimization only. No cost-based optimization for batch queries. No adaptive execution. Optimizer tuned for streaming plans (continuous queries), not one-shot batch scans. Plans are correct but not as efficient. |
| **Join strategies** | Broadcast join (small table fits in memory), sort-merge join (large-large), shuffle hash join, broadcast nested loop join. Optimizer picks automatically based on table sizes. AQE can switch strategies mid-execution if statistics differ from estimates. | Broadcast join (manual hint or size-based), hash join, sort-merge join. Fewer strategies. No automatic broadcast candidate detection. No runtime strategy switching. May need manual tuning for large joins. |
| **Shuffle / data exchange** | Tungsten engine: off-heap memory, code generation, cache-friendly data layout. Shuffle writes are sorted and compressed. AQE coalesces small shuffle partitions automatically. Decades of optimization. | Standard Java serialization with Flink's memory manager. Batch shuffle (blocking exchange) is less optimized than Flink's streaming shuffle (pipelined). No automatic partition coalescing. |
| **Vectorized Parquet reads** | Yes. Parquet -> Arrow columnar batches -> Tungsten. Processes thousands of rows per function call. Eliminates per-row overhead. | No. Reads Parquet row-by-row, not in columnar batches. Per-row deserialization overhead on large scans. |
| **Code generation** | Whole-stage code generation compiles entire query plan stages to JVM bytecode. Eliminates virtual dispatch, enables CPU pipelining. 10-100x faster than interpreted execution for complex queries. | No whole-stage code generation for batch. Interpreted execution. Each operator is a separate function call per row. |
| **Performance on large Parquet scans** | Vectorized reads + code generation + AQE. 100GB Parquet scan + aggregation in seconds on a modest cluster. Industry standard for data lake analytics. | Row-based + interpreted. Same 100GB scan: expect 3-10x slower depending on query complexity. Adequate for small-medium datasets, not competitive at scale. |
| **Schema evolution** | `spark.read.option("mergeSchema", true).parquet(path)` handles added/removed/renamed columns across Parquet files. Reads union of all schemas. Missing columns filled with null. Production-safe. | Schema must be defined upfront in table DDL. If file schema doesn't match DDL: errors or silent null fills depending on config. No automatic schema merging across files. |
| **Multi-file / multi-directory reads** | `spark.read.parquet("/data/2024/", "/data/2025/")` reads multiple directories as single dataset. Glob patterns: `/data/*/month=0[1-3]/*.parquet`. Seamless. | Requires DDL or programmatic `FileSource` with path patterns. Glob patterns supported in connector options but less ergonomic. More configuration for the same result. |
| **Nested / complex types in SQL** | Full support. Parquet `STRUCT`, `ARRAY`, `MAP` map to nested DataFrame columns. SQL: `SELECT address.city FROM users`. `EXPLODE` flattens arrays. `TRANSFORM` for map operations. Rich nested data manipulation. | Complex types supported via `ROW`, `ARRAY`, `MAP`. SQL access works (`SELECT address.city`). `UNNEST` instead of `EXPLODE`. Fewer built-in functions for nested data. |
| **Write back to Parquet** | `df.write.mode("overwrite").partitionBy("year", "month").parquet(path)`. Also: `insertInto`, `saveAsTable`. Delta Lake / Iceberg transactional writes with compaction, Z-ordering, vacuum -- all built in. | `INSERT INTO` via SQL or `TableResult`. Partition writes work. No compaction, no Z-ordering, no vacuum. Functional but lacks write-side optimizations. |
| **Data lake formats** | Delta Lake (native), Apache Iceberg (native), Apache Hudi (native). ACID transactions, time travel, schema evolution, compaction -- all first-class. Spark is THE compute engine for lakehouse architectures. | Iceberg connector exists. Delta Lake is community/experimental. Hudi has a Flink connector. All work but less mature than Spark integrations. Flink's lakehouse strength is streaming ingestion, not batch querying. |
| **UDF registration and use** | `spark.udf.register("enrich", func, returnType)`. Immediately usable in SQL: `SELECT enrich(col) FROM table`. Python UDFs via pandas. Type-safe with schema inference. | `tableEnv.createTemporaryFunction("enrich", EnrichFunction.class)`. Must implement `ScalarFunction` with `eval()` method and `@DataTypeHint` annotations. More boilerplate, same end result. |
| **Nexflow L3/L4 reuse** | Transform functions (L3) register as Spark UDFs or use in `.map()` on DataFrames. Decision tables (L4) callable inside `map()`. Pure Java logic -- no Flink dependency -- works directly. | Native target. L3 = Flink `MapFunction`. L4 = `evaluate()` in pipeline. This is what the generators produce today. |
| **Deployment model** | Ephemeral. `spark-submit` -> cluster spins up -> processes -> shuts down. YARN, Kubernetes, standalone, Databricks managed. Pay for compute only during batch window. | Persistent. Flink clusters typically run 24/7 for streaming. Batch on same cluster competes for resources. Dedicated batch cluster is wasteful (idle between runs). Operationally awkward for batch-only. |
| **Cloud cost model** | Optimized by every cloud provider. EMR, Dataproc, Databricks: spot instances, auto-scaling, per-job billing. Batch cost = (cluster size) x (job duration). Well-understood. | No cloud provider optimizes Flink for batch-only. Continuous cluster = continuous cost. Using Flink for batch SQL on Parquet means paying for always-on infrastructure for periodic workloads. |
| **Maturity for this use case** | 10+ years. Every Fortune 500 data team uses Spark SQL on Parquet. Billions of queries per day across the industry. The de facto standard for data lake analytics. | 3-4 years of serious batch support. Works correctly but far fewer production deployments for batch Parquet SQL specifically. You'd be an early adopter at scale. |

## Verdict

Spark is purpose-built for batch SQL on Parquet. Flink can do it but with:
- 3-10x slower performance (no vectorized reads, no code generation, no AQE)
- More configuration (manual DDL vs automatic discovery)
- Fewer SQL features (no CUBE/ROLLUP/PIVOT, limited subqueries)
- Awkward deployment model (persistent cluster for periodic batch)
- Higher cloud cost (always-on vs ephemeral)

## Nexflow Implication

| ProcDSL Mode | Target Runtime | Rationale |
|---|---|---|
| `mode stream` | Apache Flink | Real-time, stateful, exactly-once, event-time processing |
| `mode batch` | Apache Spark | SQL analytics, Parquet/CSV, data lake, ephemeral compute |
| `mode micro_batch` | Spark Structured Streaming | Near-real-time with batch semantics, simpler than Flink |

The grammar already defines all three modes. L3 transforms and L4 rules are pure Java
logic reusable by both Spark and Flink. Only the job orchestration layer (sources, sinks,
SQL integration, DAG construction) differs between targets.
