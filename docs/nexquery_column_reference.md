# NexQuery Column Reference

All properties computed by the 8 intelligence layers and shown in query results.

---

## Identity

| Column | Type | Layer | Meaning |
|---|---|---|---|
| **name** | string | -- | COBOL program name (compilation unit / PROGRAM-ID) |
| **kind** | string | -- | Node type: program, paragraph, field, copybook, file, table, rule |
| **program** | string | -- | Parent program name (for paragraphs and fields) |

---

## Layer 1: Structural Intelligence

| Column | Type | Range | Meaning |
|---|---|---|---|
| **paragraph_count** | int | 0+ | Number of paragraphs/sections in the PROCEDURE DIVISION |
| **field_count** | int | 0+ | Number of data items in WORKING-STORAGE, LINKAGE, LOCAL-STORAGE |
| **copybook_count** | int | 0+ | Number of COPY members referenced |
| **call_count** | int | 0+ | Number of distinct programs this one CALLs |
| **file_count** | int | 0+ | Number of FD/SD file descriptors accessed |
| **complexity** | float | 0.0-10.0 | Composite score: paragraphs (max 3) + calls (max 2) + fields (max 3) + files (max 2) |
| **program_class** | string | batch, online, subprogram, utility, unknown | Heuristic classification based on callers, callees, files |

---

## Layer 2: Control Flow Intelligence

| Column | Type | Range | Meaning |
|---|---|---|---|
| **is_entry_program** | bool | true/false | True if no other program CALLs this one (top-level entry point) |
| **entry_points** | list | [] | Paragraph names with no incoming PERFORMs (execution starts here) |
| **dead_paragraph_count** | int | 0+ | Paragraphs never PERFORMed and not performing others (unreachable) |
| **max_call_depth** | int | 0+ | Longest CALL chain from this program (A calls B calls C = depth 2) |
| **is_entry_point** | bool | paragraph | True if no other paragraph PERFORMs this one |
| **perform_count** | int | paragraph | Number of paragraphs this one PERFORMs |
| **performed_by_count** | int | paragraph | Number of paragraphs that PERFORM this one |
| **is_dead** | bool | paragraph | True if unreachable (no incoming and no outgoing PERFORMs) |

---

## Layer 3: Data Flow Intelligence

| Column | Type | Range | Meaning |
|---|---|---|---|
| **read_count** | int | field | Number of paragraphs that read this field |
| **write_count** | int | field | Number of paragraphs that write this field |
| **access_mode** | string | field | read-only, write-only, read-write, unused |
| **field_role** | string | field | input (read-only), output (write-only), accumulator (more writes than reads), temp (balanced), unused |
| **reads_count** | int | paragraph | Number of fields this paragraph reads |
| **writes_count** | int | paragraph | Number of fields this paragraph writes |
| **data_coupling** | float | 0.0-1.0 | How much paragraphs share fields. 0 = no sharing, 1 = all paragraphs touch all fields |

---

## Layer 4: Inter-Program Intelligence

| Column | Type | Range | Meaning |
|---|---|---|---|
| **naming_prefix** | string | 4 chars | First 4 characters of program name (cluster hint: CBTR, COPA, etc.) |
| **cluster** | string | 4 chars or empty | Naming-based cluster. Set only if 2+ programs share the prefix |
| **copybook_coupling** | list | [] | Programs that share at least one copybook with this program |
| **shared_copybook_count** | int | 0+ | Number of copybooks shared with other programs |
| **user_count** | int | copybook | Number of programs that COPY this member |
| **is_shared** | bool | copybook | True if used by more than one program |

---

## Layer 5: External Interface Intelligence

| Column | Type | Range | Meaning |
|---|---|---|---|
| **has_file_io** | bool | true/false | Program accesses at least one file (OPEN/READ/WRITE) |
| **has_sql** | bool | true/false | Program contains EXEC SQL statements |
| **file_inventory** | list | [] | Names of files (FD entries) this program accesses |
| **table_inventory** | list | [] | Names of SQL tables this program touches |
| **interface_count** | int | 0+ | Total external interfaces: files + tables |
| **interface_complexity** | float | 0.0-10.0 | Weighted score: files (1 point each) + tables (2 points each), capped at 10 |
| **accessor_count** | int | file/table | Number of programs accessing this file or table |
| **sql_operations** | list | table | SQL operation types: SelectInto, Insert, Update, Delete, etc. |

---

## Layer 6: Business Logic Intelligence

| Column | Type | Range | Meaning |
|---|---|---|---|
| **rule_count** | int | 0+ | Number of extracted business rules (from DSL emitters) |
| **has_rules** | bool | true/false | Whether any business rules were extracted |
| **business_density** | float | 0.0+ | Rules per paragraph. Higher = more business logic per unit of code |

---

## Layer 7: Dependency and Impact Intelligence

| Column | Type | Range | Meaning |
|---|---|---|---|
| **dependency_count** | int | 0+ | Programs this one transitively depends on (forward CALL chain) |
| **impact_radius** | int | 0+ | Programs affected if this one changes (reverse CALL chain + copybook sharing) |
| **risk_score** | float | 0.0-10.0 | Composite: 30% complexity + 20% interfaces + 20% coupling + 30% impact |
| **migration_wave** | int | 0+ | Topological order for migration: wave 0 = no deps, wave 1 = depends on wave 0, etc. |

---

## Layer 8: Pattern and Idiom Intelligence

| Column | Type | Range | Meaning |
|---|---|---|---|
| **fingerprint** | string | e.g. P0C0FLIM | Structural fingerprint for similarity. Format: P{para}C{call}F{field}I{interface}. Same fingerprint = similar structure |
| **pattern_class** | string | | Detected structural pattern |
| **era** | string | simple, structured, modern | Technology era based on features used |
| **similar_programs** | list | [] | Programs with the same fingerprint (structurally similar) |

### pattern_class values

| Value | Meaning |
|---|---|
| **batch-io** | File I/O without business rules -- pure data movement |
| **batch-read-process-write** | File I/O with business rules -- classic batch processing |
| **batch-db-io** | Both file I/O and SQL -- hybrid batch program |
| **db-crud** | SQL only, no file I/O -- database operations |
| **utility** | No calls, few paragraphs -- helper/library program |
| **unknown** | Does not match any known pattern |

### era values

| Value | Meaning |
|---|---|
| **simple** | 5 or fewer paragraphs, no SQL -- small utility or copy-like program |
| **structured** | More than 5 paragraphs, no SQL -- classic structured COBOL |
| **modern** | Uses EXEC SQL -- later-generation COBOL with database integration |

### fingerprint encoding

Each dimension is bucketed: 0 (zero), S (1-3), M (4-10), L (11-30), X (31+).
Example: `PMC0FLIM` = Paragraphs Medium, Calls zero, Fields Large, Interfaces Medium.
Programs with identical fingerprints can be migrated with the same recipe.
