# Copybook-to-Relational Mapping Strategy

> From COBOL record layouts to normalized relational (and MongoDB) schemas.

## Problem Statement

Enterprise COBOL systems store data in two fundamentally different ways:

1. **Relational (DB2/IMS-DB)** -- Schema already exists in the database catalog.
   No decomposition needed. DCLGEN copybooks map 1:1 to table DDL.

2. **Flat records (VSAM/sequential/CICS COMMAREA)** -- The copybook IS the
   only schema definition. A single 01-level record can contain 8000+ fields
   representing 70+ logical entities packed into one contiguous byte buffer.

This document addresses case 2: decomposing flat-file copybook records into
normalized relational entities suitable for migration to RDBMS or MongoDB.

## Data Source Classification

Before decomposition, classify each data structure by its origin:

| Source | Schema Authority | Action |
|--------|-----------------|--------|
| EXEC SQL (DB2) | DB2 catalog / DCLGEN copybook | 1:1 mapping, no decomposition |
| EXEC DLI (IMS) | DBD/PSB definitions | Segment -> table, PCB -> FK |
| VSAM KSDS/ESDS/RRDS | Copybook record layout | Decompose via OCCURS/REDEFINES rules below |
| Sequential file (FD) | Copybook record layout | Decompose via OCCURS/REDEFINES rules below |
| CICS COMMAREA | Linkage section copybook | Interface contract -> DTO/API schema |
| Working Storage (internal) | Inline or COPY'd definitions | May not need persistence schema |

### DB2 Path (trivial)

When COBOL uses `EXEC SQL`, the relational schema already exists:

```
DCLGEN copybook (DCLTABLE1.cpy)  -->  CREATE TABLE (DDL)
EXEC SQL SELECT ... INTO :host-vars  -->  Column mapping is explicit
```

The host variable names in EXEC SQL map directly to column names. The DCLGEN
copybook IS the relational schema in COBOL form. Migration = export DDL + data.

### IMS Path (structured)

IMS databases have explicit hierarchical schemas (DBD/PSB):

```
DBD segment  -->  Table (one per segment type)
PCB/SSA      -->  Foreign key relationships
OCCURS in segment  -->  Child table (1:N)
```

IMS-to-relational is a well-understood transformation with IBM tooling.

### Flat File Path (the hard problem)

VSAM/sequential records have no external schema. The copybook defines:
- Field positions (byte offsets)
- Field types (PIC clauses)
- Hierarchical grouping (level numbers)
- Array structures (OCCURS)
- Variant records (REDEFINES)

But it does NOT define:
- Entity boundaries (which fields form a logical entity)
- Relationships (foreign keys between entities)
- Normalization (which repeated groups should be separate tables)

---

## OCCURS -> Parent/Child Tables

OCCURS is the COBOL mechanism for repeating groups. In relational terms,
every OCCURS group becomes a **child table** with a foreign key back to
the parent record. This is the fundamental 1:N normalization.

### The Pattern

```cobol
01  WS-ORDER-RECORD.
    05  WS-ORDER-ID            PIC X(10).
    05  WS-CUSTOMER-NAME       PIC X(30).
    05  WS-LINE-ITEM-COUNT     PIC 9(3).
    05  WS-LINE-ITEMS OCCURS 50 TIMES
        DEPENDING ON WS-LINE-ITEM-COUNT.
        10  WS-ITEM-SKU        PIC X(12).
        10  WS-ITEM-QTY        PIC 9(5).
        10  WS-ITEM-PRICE      PIC S9(7)V99 COMP-3.
        10  WS-ITEM-DISCOUNT   PIC S9(3)V99 COMP-3.
```

### Relational Output

**Parent table** (from the 01-level, excluding OCCURS children):

```sql
CREATE TABLE ws_order_record (
    ws_order_id         VARCHAR(10) PRIMARY KEY,
    ws_customer_name    VARCHAR(30),
    ws_line_item_count  INTEGER
);
```

**Child table** (from the OCCURS group):

```sql
CREATE TABLE ws_line_items (
    ws_order_id         VARCHAR(10) NOT NULL,  -- FK to parent
    sequence_num        INTEGER NOT NULL,       -- occurrence index (1-based)
    ws_item_sku         VARCHAR(12),
    ws_item_qty         INTEGER,
    ws_item_price       DECIMAL(9, 2),
    ws_item_discount    DECIMAL(5, 2),
    PRIMARY KEY (ws_order_id, sequence_num),
    FOREIGN KEY (ws_order_id) REFERENCES ws_order_record(ws_order_id)
);
```

### Rules

| COBOL Pattern | Relational Mapping |
|--------------|-------------------|
| `OCCURS N TIMES` (fixed) | Child table, max N rows per parent |
| `OCCURS N DEPENDING ON field` | Child table, actual count from field |
| `OCCURS` on elementary item | Array column or child table (threshold: >1 field = table) |
| Nested OCCURS (OCCURS inside OCCURS) | Grandchild table with compound FK |
| `INDEXED BY idx` | Suggests key-based access, add INDEX on child table |
| `KEY IS field` on OCCURS | That field is the natural PK of the child table |

### Nested OCCURS (Grandchild)

```cobol
01  WS-DEPT-TABLE.
    05  WS-DEPT OCCURS 10 TIMES.
        10  WS-DEPT-CODE       PIC X(4).
        10  WS-EMPLOYEE OCCURS 100 TIMES.
            15  WS-EMP-ID      PIC X(10).
            15  WS-EMP-NAME    PIC X(30).
```

Produces three tables:

```
ws_dept_table (parent)        -- no PK from COBOL, synthetic
ws_dept (child)               -- FK to parent, PK = (parent_id, dept_seq)
ws_employee (grandchild)      -- FK to ws_dept, PK = (parent_id, dept_seq, emp_seq)
```

Or with COBOL-named keys:

```
ws_dept.ws_dept_code          -- natural key if unique
ws_employee.ws_emp_id         -- natural key if unique
```

### Primary Key Identification for OCCURS

Sources of primary key information, in priority order:

1. **RECORD KEY IS** from FILE SECTION SELECT clause
   - AST: `FileDescription.record_key: Option<String>`
   - Already parsed by transpiler
   - This IS the primary key for VSAM KSDS files

2. **ALTERNATE RECORD KEY** from SELECT clause
   - AST: `FileDescription.alternate_keys: Vec<String>`
   - These are unique indexes (unless WITH DUPLICATES)

3. **KEY IS field** on OCCURS clause
   - Explicit key declaration for SEARCH/SEARCH ALL
   - Natural primary key of the child table

4. **INDEXED BY** on OCCURS
   - Indicates key-based access patterns
   - The index variable names hint at usage

5. **Naming convention heuristics**
   - Fields ending in -ID, -KEY, -CODE, -NUMBER, -NUM, -NO
   - First field in a group (COBOL convention)
   - Fields used in SEARCH ALL (binary search = unique key)

6. **PROCEDURE DIVISION analysis**
   - Fields used in READ ... KEY IS
   - Fields used in START ... KEY IS
   - Fields used in SEARCH ALL ... WHEN field = value

### When No Key Is Found

If no primary key can be identified from the above sources:

- Emit a synthetic surrogate key (auto-increment)
- Add a review note: "No primary key detected -- synthetic key added"
- The domain expert assigns the real key during review

---

## REDEFINES -> Variant Tables with Discriminator

REDEFINES is the COBOL mechanism for variant/union records: multiple
field layouts overlaid on the same byte range. In relational terms,
each REDEFINES variant becomes a **separate table** linked by a
**discriminator column** that determines which layout is active.

### Existing Implementation: cobol-data Viewer

The data viewer (crates/cobol-data/) already has a complete REDEFINES
handling pipeline that this strategy builds on:

```
DataEntry tree
  |
  v
[extract_redefines_groups()]     -- redefines.rs
  Scans children for REDEFINES clauses
  Groups variants by base field
  Handles column-72 truncation (prefix matching)
  |
  v
RedefinesGroup {
    base_field: "BASE-DATA",
    byte_offset, byte_length,
    variants: [RedefinesVariant { name, fields }],
    discriminator: None
}
  |
  v
[detect_discriminators()]        -- discriminator.rs
  Walks PROCEDURE DIVISION AST
  Detects 3 patterns (see below)
  Assigns confidence scores
  |
  v
Discriminator {
    field: "REC-TYPE",
    pattern_type: DirectIf | EvaluateWhen | Level88,
    rules: [{ value: "P", selects: "PERSONAL" }],
    confidence: High | Medium | Low | Unresolved
}
```

### Discriminator Detection Patterns

The data viewer detects discriminators via three AST patterns,
all of which map directly to the `type` column in the relational model:

**Pattern 1: EVALUATE (highest signal)**

```cobol
EVALUATE ACCT-TYPE
    WHEN "P"
        MOVE PERS-NAME TO WS-OUTPUT    (references PERSONAL variant fields)
    WHEN "B"
        MOVE COMP-NAME TO WS-OUTPUT    (references BUSINESS variant fields)
END-EVALUATE
```

Detection: EVALUATE subject = discriminator field. WHEN values = discriminator
rules. Field references in each branch matched to variant field lists.
Confidence: HIGH.

**Pattern 2: IF/ELSE IF chain**

```cobol
IF ACCT-TYPE = "P"
    ... (references PERSONAL variant fields)
ELSE IF ACCT-TYPE = "B"
    ... (references BUSINESS variant fields)
END-IF
```

Detection: Comparison field = discriminator. Literal value = rule.
Then/else body field references matched to variants.
Follows PERFORM chains up to 3 levels deep.
Confidence: HIGH.

**Pattern 3: Level-88 condition names (indirect)**

```cobol
05  ACCT-TYPE        PIC X(1).
    88  IS-PERSONAL  VALUE "P".
    88  IS-BUSINESS  VALUE "B".

IF IS-PERSONAL
    ... (references PERSONAL variant fields)
END-IF
```

Detection: Condition name resolved to parent field via 88-level map.
Condition VALUE = discriminator rule. Indirection adds uncertainty.
Confidence: MEDIUM.

**When no discriminator is found**: Confidence = UNRESOLVED.
The variant tables are still created but flagged for SME review.

### The COBOL Pattern

```cobol
01  WS-PAYMENT-RECORD.
    05  WS-PAY-TYPE            PIC X(1).
        88  WS-PAY-CHECK       VALUE "C".
        88  WS-PAY-ACH         VALUE "A".
        88  WS-PAY-WIRE        VALUE "W".
    05  WS-PAY-AMOUNT          PIC S9(9)V99 COMP-3.
    05  WS-PAY-DETAILS         PIC X(50).
    05  WS-PAY-CHECK-DETAIL REDEFINES WS-PAY-DETAILS.
        10  WS-CHECK-NUMBER    PIC X(10).
        10  WS-CHECK-BANK      PIC X(20).
        10  WS-CHECK-ROUTING   PIC 9(9).
        10  FILLER             PIC X(11).
    05  WS-PAY-ACH-DETAIL REDEFINES WS-PAY-DETAILS.
        10  WS-ACH-ACCOUNT     PIC X(17).
        10  WS-ACH-ROUTING     PIC 9(9).
        10  WS-ACH-TYPE        PIC X(1).
        10  FILLER             PIC X(23).
```

### Relational Output

**Base table** (shared columns + discriminator):

```sql
CREATE TABLE ws_payment_record (
    payment_id          INTEGER PRIMARY KEY,    -- synthetic (no COBOL PK)
    ws_pay_type         CHAR(1) NOT NULL,       -- discriminator
    ws_pay_amount       DECIMAL(11, 2),         -- shared across all variants
    CONSTRAINT chk_pay_type CHECK (ws_pay_type IN ('C', 'A', 'W'))
);
```

**Variant table: CHECK** (when ws_pay_type = 'C'):

```sql
CREATE TABLE ws_pay_check_detail (
    payment_id          INTEGER PRIMARY KEY,
    ws_check_number     VARCHAR(10),
    ws_check_bank       VARCHAR(20),
    ws_check_routing    INTEGER,
    FOREIGN KEY (payment_id) REFERENCES ws_payment_record(payment_id)
);
-- FILLER fields dropped
```

**Variant table: ACH** (when ws_pay_type = 'A'):

```sql
CREATE TABLE ws_pay_ach_detail (
    payment_id          INTEGER PRIMARY KEY,
    ws_ach_account      VARCHAR(17),
    ws_ach_routing      INTEGER,
    ws_ach_type         CHAR(1),
    FOREIGN KEY (payment_id) REFERENCES ws_payment_record(payment_id)
);
```

### Rules

| COBOL Pattern | Relational Mapping |
|--------------|-------------------|
| Base field (redefined) | Omit from schema (raw bytes, not meaningful) |
| Each REDEFINES variant | Separate table with 1:1 FK to parent |
| Discriminator field | Column in parent + CHECK constraint from 88-levels |
| Level-88 on discriminator | CHECK constraint enum values |
| Shared fields (before REDEFINES) | Columns in parent table |
| FILLER in variants | Dropped (padding, not data) |
| No discriminator found | Tables still created, flagged UNRESOLVED for SME |

### Confidence-Based Handling

| Confidence | Action |
|-----------|--------|
| HIGH (IF/EVALUATE match) | Auto-generate variant tables + discriminator column |
| MEDIUM (88-level indirect) | Auto-generate, flag for verification |
| LOW (nested conditions) | Generate with warning, needs SME review |
| UNRESOLVED | Generate base table only, emit all variants as review candidates |

### Multi-Level REDEFINES

When REDEFINES chains exist (A REDEFINES B, C REDEFINES B):

- All variants share the same byte range
- All get their own table
- All FK to the same parent
- Discriminator field (if found) selects between ALL variants

---

## Primary Key Strategy

### Priority Order for PK Detection

| Priority | Source | AST Location | Confidence |
|----------|--------|-------------|-----------|
| 1 | `RECORD KEY IS field` (SELECT clause, ENVIRONMENT DIV) | FileDescription.record_key | Definitive |
| 2 | `ALTERNATE RECORD KEY` | FileDescription.alternate_keys | Unique index |
| 3 | `KEY IS field` on OCCURS | DataEntry (future) | Definitive for child |
| 4 | `SEARCH ALL ... WHEN field =` | ProcedureDivision statements | High (binary search = unique) |
| 5 | `READ ... KEY IS field` | ProcedureDivision statements | High (keyed access) |
| 6 | First field in 01-level group | DataEntry.children[0] | Medium (COBOL convention) |
| 7 | Field name heuristic (-ID, -KEY, -NUM) | DataEntry.name | Low |
| 8 | Synthetic surrogate | Generated | Fallback |

### VSAM File Types and Keys

| VSAM Type | Key Characteristic | PK Source |
|-----------|-------------------|-----------|
| KSDS (Keyed Sequential) | Has primary key | RECORD KEY IS (mandatory) |
| ESDS (Entry Sequenced) | No key (sequential) | RBA or synthetic |
| RRDS (Relative Record) | Relative record number | RELATIVE KEY IS |
| Alternate Index (AIX) | Secondary access path | ALTERNATE RECORD KEY |

For KSDS files, the primary key is always known from the SELECT clause.
This is the most common VSAM type in enterprise systems.

---

## Nested OCCURS: Grandchild and Deeper Entities

OCCURS inside OCCURS is how COBOL represents multi-level hierarchies.
The same decomposition rules apply recursively at every depth.

### Example: 3-Level Hierarchy

```cobol
01  WS-COMPANY-RECORD.
    05  WS-COMPANY-ID          PIC X(10).
    05  WS-COMPANY-NAME        PIC X(40).
    05  WS-DEPT OCCURS 20 TIMES.                     <-- child (L1)
        10  WS-DEPT-CODE       PIC X(4).
        10  WS-DEPT-NAME       PIC X(25).
        10  WS-EMPLOYEE OCCURS 100 TIMES.             <-- grandchild (L2)
            15  WS-EMP-ID      PIC X(10).
            15  WS-EMP-NAME    PIC X(30).
            15  WS-SKILL OCCURS 10 TIMES.              <-- great-grandchild (L3)
                20  WS-SKILL-CODE  PIC X(8).
                20  WS-SKILL-LEVEL PIC 9(2).
```

### Relational Output (4 tables)

```sql
-- Parent: 01-level record
CREATE TABLE ws_company_record (
    ws_company_id    VARCHAR(10) PRIMARY KEY,  -- from RECORD KEY IS
    ws_company_name  VARCHAR(40)
);

-- Child (L1): first OCCURS
CREATE TABLE ws_dept (
    ws_company_id    VARCHAR(10) NOT NULL,     -- FK to parent
    dept_seq         INTEGER NOT NULL,          -- occurrence index
    ws_dept_code     VARCHAR(4),
    ws_dept_name     VARCHAR(25),
    PRIMARY KEY (ws_company_id, dept_seq),
    FOREIGN KEY (ws_company_id) REFERENCES ws_company_record(ws_company_id)
);

-- Grandchild (L2): nested OCCURS
CREATE TABLE ws_employee (
    ws_company_id    VARCHAR(10) NOT NULL,     -- FK chain to root
    dept_seq         INTEGER NOT NULL,          -- FK to parent
    emp_seq          INTEGER NOT NULL,          -- occurrence index
    ws_emp_id        VARCHAR(10),
    ws_emp_name      VARCHAR(30),
    PRIMARY KEY (ws_company_id, dept_seq, emp_seq),
    FOREIGN KEY (ws_company_id, dept_seq) REFERENCES ws_dept(ws_company_id, dept_seq)
);

-- Great-grandchild (L3): doubly nested OCCURS
CREATE TABLE ws_skill (
    ws_company_id    VARCHAR(10) NOT NULL,     -- FK chain
    dept_seq         INTEGER NOT NULL,
    emp_seq          INTEGER NOT NULL,          -- FK to parent
    skill_seq        INTEGER NOT NULL,          -- occurrence index
    ws_skill_code    VARCHAR(8),
    ws_skill_level   INTEGER,
    PRIMARY KEY (ws_company_id, dept_seq, emp_seq, skill_seq),
    FOREIGN KEY (ws_company_id, dept_seq, emp_seq)
        REFERENCES ws_employee(ws_company_id, dept_seq, emp_seq)
);
```

### Rules (apply identically at every depth)

| Depth | PK Source (priority order) | FK |
|-------|---------------------------|----|
| Parent (01) | `RECORD KEY IS` from SELECT | None (root) |
| Child (OCCURS L1) | `KEY IS` on OCCURS, naming heuristic, or synthetic seq | Parent PK |
| Grandchild (OCCURS L2) | Same: KEY IS, heuristic, or seq | Parent PK + child key |
| Great-grandchild (OCCURS L3) | Same rules | Compound FK grows by one segment |
| ... | Pattern repeats | FK chain lengthens |

The compound FK always = parent's full PK + this level's sequence/key.
Depth doesn't change the rule, it just lengthens the compound key.

### Natural Keys vs Synthetic Sequences

When an OCCURS has `KEY IS field` or a clear naming pattern (-ID, -CODE):

```sql
-- Natural key: ws_dept_code is unique within parent
PRIMARY KEY (ws_company_id, ws_dept_code)    -- instead of dept_seq
```

When no natural key is identifiable:

```sql
-- Synthetic sequence: occurrence index (1-based)
PRIMARY KEY (ws_company_id, dept_seq)
```

### REDEFINES Inside OCCURS

A REDEFINES within an OCCURS group means the child table has variant rows:

```cobol
05  WS-TRANSACTION OCCURS 500 TIMES.
    10  WS-TXN-TYPE      PIC X(1).
    10  WS-TXN-DATA      PIC X(40).
    10  WS-TXN-PURCHASE REDEFINES WS-TXN-DATA.
        15  WS-PURCH-AMT PIC S9(9)V99.
        15  WS-PURCH-SKU PIC X(12).
    10  WS-TXN-PAYMENT REDEFINES WS-TXN-DATA.
        15  WS-PAY-AMT   PIC S9(9)V99.
        15  WS-PAY-METHOD PIC X(1).
```

This produces: one child table (ws_transaction) + variant tables
(ws_txn_purchase, ws_txn_payment) with discriminator (ws_txn_type).
Same REDEFINES rules apply, just at the child level instead of root.

### Practical Limits

Enterprise COBOL rarely exceeds 3 levels of OCCURS nesting:
- Level 1: Common (arrays, tables, repeating groups)
- Level 2: Occasional (line items in an order in a batch)
- Level 3: Rare (skills per employee per department)
- Level 4+: Extremely rare, typically only in stress tests

---

## Sequential Files and Key Identification

Not all VSAM files have explicit keys. The key availability depends on
the file organization and how the program accesses it.

### Key Availability and Schema Pattern by File Type

The file organization and access pattern determine both the key source
AND the appropriate schema mutation pattern -- no field-name guessing needed.

| File Type | Access Mode | Key in COBOL? | Key in Reality? | Schema Pattern |
|-----------|-------------|--------------|-----------------|----------------|
| VSAM KSDS (random) | RANDOM/DYNAMIC | Yes: `RECORD KEY IS` | Same field | `master_data` |
| VSAM KSDS (batch) | SEQUENTIAL | Yes: still declared | Same field | `master_data` |
| VSAM ESDS | SEQUENTIAL | No (ESDS has no key) | RBA or timestamp + seq | `event_log` or `immutable_ledger` |
| VSAM RRDS | RANDOM/SEQ | `RELATIVE KEY IS` (slot#) | Slot number = surrogate | `reference_data` |
| Sequential input (tape) | SEQUENTIAL | No clause exists | Convention (first field) | `master_data` or `reference_data` |
| Sequential output/report | SEQUENTIAL | No | Often none (output) | `document` or `aggregate` |
| CICS COMMAREA | N/A (memory) | No file key | Interface contract | `command` / `response` |
| DB2 (EXEC SQL) | N/A (relational) | PK in DDL | Column constraint | Inherit from DDL |
| IMS (EXEC DLI) | Hierarchical | Segment key in DBD | Segment key | `master_data` per segment |

**Pattern detection rules (automatable from SELECT clause):**

| COBOL Signal | Detected Pattern | Rationale |
|-------------|-----------------|-----------|
| `ORGANIZATION IS INDEXED` | `master_data` | Keyed, updated in place, random access |
| `ORGANIZATION IS SEQUENTIAL` + OPEN OUTPUT | `document` / `aggregate` | Generated output, not a live entity |
| `ORGANIZATION IS SEQUENTIAL` + OPEN INPUT | `master_data` or `reference_data` | Batch input, likely master extract |
| `ORGANIZATION IS RELATIVE` | `reference_data` | Slot-based lookup table |
| ESDS (no ORGANIZATION IS INDEXED/RELATIVE) | `event_log` | Append-only, no update, no delete |
| ESDS + financial naming (LEDGER, JOURNAL, TRANS) | `immutable_ledger` | Append-only financial records |
| LINKAGE SECTION / DFHCOMMAREA | `command` / `response` | CICS inter-program message |
| `EXEC SQL` present | Inherit from DDL | Relational schema already defined |

This eliminates the current field-name heuristic for pattern detection
(checking for "timestamp", "status", "flag" in field names). The file
organization is a stronger, more reliable signal.

**Critical insight**: Even when a KSDS file is accessed sequentially
(batch processing), the SELECT statement still declares `RECORD KEY IS`.
The developer must always specify it for ORGANIZATION IS INDEXED,
regardless of ACCESS MODE. So we get the PK for free in ALL KSDS programs.

### Truly Keyless Scenarios

Programs without `RECORD KEY IS`:

1. **ESDS (Entry-Sequenced Data Set)** -- append-only logs. No user key.
   Internal RBA (Relative Byte Address) is the only identifier.

2. **Sequential files** -- `ORGANIZATION IS SEQUENTIAL` or no ORGANIZATION
   clause. No key mechanism exists in the COBOL syntax.

3. **WORKING-STORAGE only records** -- not files at all, just in-memory
   structures defined via copybooks.

### PK Discovery for Keyless Files

When no `RECORD KEY IS` exists, apply these strategies in order:

| Priority | Strategy | How |
|----------|----------|-----|
| 1 | Cross-reference with other programs | Another program may read the SAME dataset as KSDS with a key declared |
| 2 | JCL/catalog lookup | VSAM catalog knows if dataset is KSDS/ESDS -- if KSDS, key is in catalog |
| 3 | First field convention | First 05-level field in the record is the key (strong COBOL convention) |
| 4 | Naming heuristic | Fields ending in -ID, -KEY, -CODE, -NUMBER, -NUM, -NO |
| 5 | PROCEDURE DIVISION analysis | Fields used in comparison/SEARCH for matching records |
| 6 | Domain expert | SME assigns the key |
| 7 | Synthetic surrogate | Auto-increment as last resort |

### Cross-Program Key Discovery

The most powerful strategy for keyless files: the same physical dataset
may be accessed by multiple programs. If ANY program declares it as KSDS
with RECORD KEY IS, that key applies to the data regardless of how other
programs access it.

```
Program A: SELECT ACCT-FILE ... ORGANIZATION INDEXED RECORD KEY IS ACCT-NUM
Program B: SELECT ACCT-FILE ... ORGANIZATION SEQUENTIAL  (no key clause)
```

Both read the same VSAM KSDS dataset. Program B doesn't declare the key
but the data still has ACCT-NUM as its primary key. Cross-referencing
programs by their ASSIGN TO physical filename reveals this.

Implementation: scan all programs' SELECT statements, group by physical
filename (ASSIGN TO), and propagate RECORD KEY IS from any program that
declares it to all programs using the same file.

---

## Decomposition Strategy for Flat Records

### Phase 1: Copybook Provenance (Automatic)

Track which COPY statement introduced each block of fields in the AST.

```cobol
01  WS-CARD-MASTER.
    COPY ACCT-MASTER.        --> Fields from ACCT-MASTER.cpy
    COPY CARDHOLDER-INFO.    --> Fields from CARDHOLDER-INFO.cpy
    COPY TRANS-HISTORY.      --> Fields from TRANS-HISTORY.cpy
```

**Rule**: Each COPY'd block = one candidate entity. The copybook name is
the entity name. This works because copybooks were designed as shared,
reusable record definitions -- they represent the enterprise's data model.

Implementation:
- Parser tracks `copybook_source: Option<String>` on each DataEntry
- Schema emitter uses copybook boundaries as entity boundaries
- Same copybook used in 50 programs -> same schema referenced everywhere

### Phase 2: OCCURS/REDEFINES Structural Rules (Automatic)

Apply the OCCURS and REDEFINES rules from the sections above:

- OCCURS group -> child table with FK + sequence_num
- REDEFINES group -> variant tables with discriminator
- Discriminator detection reuses cobol-data pipeline
- RECORD KEY IS -> primary key (from FILE SECTION AST)

### Phase 3: Cross-Reference Validation

Validate entity boundaries against PROCEDURE DIVISION usage:

- Fields always read/written together -> likely same entity
- Fields from different COPY blocks linked by MOVE -> FK relationship
- OCCURS traversal in PERFORM VARYING -> confirms parent-child
- SEARCH ALL on OCCURS -> confirms key field
- IF/EVALUATE on discriminator field -> confirms REDEFINES structure

### Phase 4: Domain Expert Review

The automated decomposition produces candidates. A domain expert confirms:
- Are these the right entity boundaries?
- Are the detected primary keys correct?
- What are the foreign key relationships between entities?
- Which OCCURS groups are lookup tables vs. child records?
- Are the discriminator mappings correct?

---

## Mapping Rules: Elementary Fields

| COBOL | Relational | Notes |
|-------|-----------|-------|
| PIC X(n) | VARCHAR(n) or CHAR(n) | CHAR if fixed-length usage |
| PIC A(n) | VARCHAR(n) | Alphabetic only |
| PIC 9(n) | INTEGER or BIGINT | Based on precision |
| PIC S9(n) | INTEGER or BIGINT | Signed |
| PIC S9(n)V9(m) | DECIMAL(n+m, m) | Exact |
| PIC S9(n)V9(m) COMP-3 | DECIMAL(n+m, m) | Packed decimal |
| PIC S9(n) COMP/COMP-4 | SMALLINT/INT/BIGINT | Binary, PIC-limited range |
| PIC S9(n) COMP-5 | SMALLINT/INT/BIGINT | Native binary, full range |
| COMP-1 | REAL/FLOAT(24) | Single precision |
| COMP-2 | DOUBLE/FLOAT(53) | Double precision |
| PIC 9(8) (date-named) | DATE | Detected by name + PIC pattern |
| PIC X(26) (timestamp-named) | TIMESTAMP | Detected by naming convention |
| Level-88 values | CHECK constraint | Enum values from condition names |
| FILLER | (omitted) | Padding, not data |

---

## Mapping Rules: Relational -> MongoDB

Once relational schema is established, MongoDB mapping is mechanical:

| Relational | MongoDB | Rule |
|-----------|---------|------|
| Table | Collection | 1:1 |
| Row | Document | 1:1 |
| Column | Field | 1:1 |
| PRIMARY KEY | _id (or compound index) | Direct |
| 1:1 variant table | Embedded subdocument | Natural fit |
| 1:N child table (few) | Embedded array | If child count < ~100 |
| 1:N child table (many) | Separate collection + reference | If unbounded |
| M:N | Array of references | Context-dependent |
| DECIMAL | NumberDecimal (128-bit) | Exact |
| VARCHAR | String | Direct |
| DATE/TIMESTAMP | ISODate | Direct |
| CHECK constraint | Schema validation rule | $jsonSchema |

### OCCURS -> MongoDB

OCCURS maps naturally to MongoDB's embedded arrays:

```json
{
  "_id": "ORD-001",
  "ws_customer_name": "SMITH",
  "ws_line_items": [
    { "ws_item_sku": "ABC123", "ws_item_qty": 5, "ws_item_price": 19.99 },
    { "ws_item_sku": "DEF456", "ws_item_qty": 2, "ws_item_price": 49.99 }
  ]
}
```

For small OCCURS (< 100 items, always accessed with parent) -> embed.
For large OCCURS or independently accessed -> separate collection.

### REDEFINES -> MongoDB

REDEFINES maps naturally to MongoDB's flexible schema:

```json
{
  "_id": "PAY-001",
  "ws_pay_type": "C",
  "ws_pay_amount": 500.00,
  "ws_pay_check_detail": {
    "ws_check_number": "1234567890",
    "ws_check_bank": "FIRST NATIONAL",
    "ws_check_routing": 123456789
  }
}
```

Only the active variant is populated in each document.
The discriminator field (`ws_pay_type`) determines which subdocument exists.
This is more natural in MongoDB than in relational (no separate tables needed).

### Embedding vs. Reference Decision

```
If child entity is:
  - Always accessed with parent -> EMBED
  - Accessed independently -> REFERENCE (separate collection)
  - Unbounded growth -> REFERENCE
  - Shared across parents -> REFERENCE
  - < 100 items AND always with parent -> EMBED
  - REDEFINES variant -> EMBED (1:1, always with parent)
```

---

## Implementation Plan

### Now (Tier 1): Flat Schema Emission

- Schema boundary = 01-level group
- All nesting flattened to field list
- Sub-groups become section comments
- OCCURS -> annotated for child table extraction
- REDEFINES -> annotated with discriminator info from cobol-data
- Full COBOL field names preserved (1:1 traceability)
- Primary key from RECORD KEY IS when available

### Next (Tier 2): Copybook Provenance

- Add `copybook_source: Option<String>` to DataEntry in parser
- COPY preprocessor tags expanded entries with source copybook name
- Schema emitter groups by copybook, not by 01-level
- Same copybook in N programs -> one shared .schema, N references

### Next (Tier 3): Cross-Program Key Discovery

- Scan all programs' SELECT statements for RECORD KEY IS / RELATIVE KEY IS
- Group programs by physical filename (ASSIGN TO clause)
- Propagate keys: if any program declares a key for a file, all programs share it
- Build file-key registry: physical_file -> (key_field, organization, access_mode)
- Covers keyless sequential programs that read KSDS datasets

### Next (Tier 4): OCCURS/REDEFINES Table Extraction

- OCCURS groups -> child .schema files with FK annotation
- REDEFINES groups -> variant .schema files with discriminator annotation
- Reuse cobol-data's `extract_redefines_groups()` and `detect_discriminators()`
- Emit parent-child relationships in schema metadata

### Later (Tier 5): EXEC SQL Direct Mapping

- Detect EXEC SQL host variables
- Map to DB2 catalog (if DDL available) or DCLGEN copybooks
- Emit schema that mirrors the existing relational structure
- No decomposition heuristics needed -- the schema is already relational

### Future (Tier 6): Cross-Reference Validation

- Analyze PROCEDURE DIVISION field access patterns
- Identify co-access clusters (fields always used together)
- Validate copybook boundaries against actual usage
- Suggest FK relationships from MOVE patterns
- Domain expert confirms via NexStudio UI

---

## Decision Record

| Decision | Rationale |
|----------|-----------|
| DB2 = 1:1 relational | Schema already exists, don't re-derive it |
| Copybook = entity boundary | Copybooks are the enterprise data dictionary |
| OCCURS = child table | Repeating groups must normalize (1NF). Parent FK + sequence_num. |
| REDEFINES = variant tables | Each overlay is a separate entity. Discriminator from cobol-data. |
| Reuse cobol-data discriminator detection | Already implemented: 3 patterns, 4 confidence levels, AST-driven |
| RECORD KEY IS = primary key | From ENVIRONMENT DIV SELECT clause. Definitive for VSAM KSDS. |
| File organization = schema pattern | KSDS->master_data, ESDS->event_log, RRDS->reference_data. Stronger than field-name heuristics. |
| Flatten deep nesting | Intermediate groups are structural, not semantic |
| Full COBOL names in schema | 1:1 traceability, single-hop mapping to target |
| Relational-first, then MongoDB | Relational forces normalization discipline |
| FILLER = dropped | Padding bytes, not data. Never migrate. |
| Cross-program key propagation | Same dataset read as KSDS by one program reveals key for all programs |
| KSDS sequential still has key | RECORD KEY IS is mandatory even with ACCESS MODE SEQUENTIAL |
| ESDS/sequential = truly keyless | Must use heuristics, cross-reference, or domain expert |
| No PK found = synthetic + review note | Safe fallback, domain expert assigns real key |

## References

- IBM Enterprise COBOL COPY Statement: https://www.ibm.com/docs/en/cobol-zos/6.3.0?topic=statements-copy-statement
- COBOL Data Structures and Record Layouts: http://www.simotime.com/simorec1.htm
- MuleSoft COBOL Copybook Format: https://docs.mulesoft.com/dataweave/latest/dataweave-formats-copybook
- Astadia COBOL System Analysis: https://www.astadia.com/blog/inside-the-engine-how-astadia-analyzes-and-maps-cobolsystems-for-modernization
- Azure Legacy Modernization Agents: https://devblogs.microsoft.com/all-things-azure/how-we-use-ai-agents-for-cobol-migration-and-mainframe-modernization/
- Broadcom COBOL Copybook: https://techdocs.broadcom.com/us/en/ca-mainframe-software/database-management/ca-idms/19-0/administrating/extract-transform-and-load/extract-data/cobol-copybook.html
- cobol-data crate (internal): crates/cobol-data/src/redefines.rs, discriminator.rs, record.rs
