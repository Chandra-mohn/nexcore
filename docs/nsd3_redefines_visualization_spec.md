# NSD3: REDEFINES Visualization Spec

## Design Principle

Data-first. Show only data values. Variant and structural information
communicated through color, column hints, and cell tooltips -- never
as extra non-data columns or tabs.

## Table Design

### Columns
- Shared fields (outside REDEFINES regions): normal columns, always same meaning
- REDEFINES region fields: position-based columns (col_1, col_2, etc.)
  - Column header: generic position label
  - Column inlay hint (muted sub-text): variant field names stacked, separated by /
  - Example: "name / company / limit" under a REDEFINES position column

### Row Color (left border, 3px)
- Each L1 variant gets a consistent hue (auto-assigned)
- Fully resolved: colored border (blue, green, purple, etc.)
- Partially resolved: orange border
- Unresolved at L1: yellow border
- No REDEFINES: no colored border

### Cell Tooltip (on hover)
Shows full context for any cell:
- Field name (qualified: VARIANT.FIELD)
- PIC clause, USAGE, byte offset, byte length
- Full cascading resolution chain with status per level
- Confidence at each level
- Raw hex bytes

Example:
```
Resolution Chain:
  L1: TYPE = "CR" -> CREDIT        [HIGH]
  L2: SUB-TYPE = "R" -> RECURRING  [HIGH]
  L3: PLAN = "M" -> MONTHLY        [MEDIUM]

Field: MONTHLY-DAY
PIC 9(2) DISPLAY  Bytes 30-31
Value: 15
```

Partially resolved:
```
Resolution Chain:
  L1: TYPE = "CR" -> CREDIT        [HIGH]
  L2: SUB-TYPE = "Z" -> ???        [UNRESOLVED]
  L3: (blocked by L2)              [--]

Showing raw bytes for L2+ region
```

## Discriminator Topologies

The engine must handle all of these. The UI design works regardless
because it treats each record as "resolved fields + confidence."

### Independent
Two+ REDEFINES groups with separate discriminators. No dependency.
Resolution order doesn't matter. Both can be resolved in parallel.

### Cascading
L2 discriminator lives inside an L1 variant. Must resolve L1 first
to find and read L2 discriminator. Chain breaks if any level unresolved.

### Cross-referencing
One discriminator affects multiple REDEFINES groups. Single field value
selects variants in different byte regions simultaneously.

### Computed
Discriminator is a compound condition (IF A > 10000 AND B = "US").
Not a simple value->variant match. Requires expression evaluation.

### 88-level indirect
Condition names map multiple values to one variant through an intermediary.
88 IS-PERSONAL VALUE "P" "I" "S" -> all three select PERSONAL variant.

### PERFORM-chained
Discriminator check happens in a called paragraph, not inline.
Requires cross-paragraph data flow analysis to detect.

## Engine Status (coqu-di detect_discriminators)

| Topology         | Supported | Confidence |
|------------------|-----------|------------|
| Direct IF        | Yes       | HIGH       |
| EVALUATE/WHEN    | Yes       | HIGH       |
| 88-level         | Yes       | MEDIUM     |
| Cascading L1     | Yes       | HIGH       |
| Cascading L2+    | Partial   | LOW        |
| Cross-referencing| No        | --         |
| Computed         | No        | --         |
| PERFORM-chained  | No        | --         |

## Engine Gaps (future work)

1. Multi-hop PERFORM chain analysis (follow discriminator across paragraphs)
2. Compound condition support (AND/OR in IF statements)
3. Cross-group mapping (one discriminator -> multiple REDEFINES regions)
4. Deep cascading (resolve L1, re-analyze for L2+ discriminators)
5. Data-driven inference: when code analysis fails, use statistical
   analysis of actual data values to suggest likely discriminators

## Implementation Notes

- UI: modify DataViewer.svelte to add inlay hints, row colors, tooltips
- Backend: dv_decode_window already returns variant + warnings per record
- Need new backend call: dv_get_redefines_info() returning group topology
- Column hint text derived from schema info (already returned by dv_attach_schema)
