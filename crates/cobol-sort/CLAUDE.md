# cobol-sort

COBOL SORT/MERGE engine. Adaptive in-memory and external merge sort.

## Key Types

- `CobolSortEngine` -- Main sort engine (adaptive: in-memory for small, external for large)
- `CobolMergeEngine` -- K-way merge via min-heap
- `SortKeySpec` -- Sort key definition (field offset, length, ascending/descending)
- `RecordComparator` -- Multi-key comparison
- `CollatingTable` -- EBCDIC/ASCII collating sequences
- `Releaser` / `Returner` -- INPUT/OUTPUT PROCEDURE interfaces

## Modules

- `sort_engine.rs` -- Main engine with adaptive switching
- `merge.rs` -- K-way merge implementation
- `in_memory.rs` / `external.rs` -- Sort variants
- `procedure.rs` -- RELEASE/RETURN for INPUT/OUTPUT PROCEDURE
- `sort_key.rs` -- Key definitions and comparators
- `collating.rs` -- Custom collating sequences
- `config.rs` -- SortConfig (memory limit, temp dir)

## Dependencies

cobol-core, cobol-types, cobol-io, rust_decimal, tempfile

## Rules

- Adaptive: switches from in-memory to external sort based on memory limit
- External sort uses tempfile-backed merge runs
- INPUT PROCEDURE: user code RELEASEs records to sort
- OUTPUT PROCEDURE: user code RETURNs records from sort
- Multi-key sorting with ASC/DESC per key
