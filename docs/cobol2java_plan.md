---

marp: true
theme: uncover
paginate: true
style: |
/* VS Code Dark Modern Theme */

*   { letter-spacing: normal !important; }
    :root {
    --bg-primary: #1f1f1f;
    --bg-secondary: #252526;
    --bg-elevated: #2d2d2d;
    --fg-primary: #cccccc;
    --fg-secondary: #9d9d9d;
    --accent-blue: #0078d4;
    --accent-blue-light: #3794ff;
    --accent-green: #4ec9b0;
    --accent-orange: #ce9178;
    --accent-yellow: #dcdcaa;
    --accent-purple: #c586c0;
    --border: #3c3c3c;
    }
    section { background: var(--bg-primary); color: var(--fg-primary); font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 18px; letter-spacing: normal; padding: 35px 50px; justify-content: flex-start; text-align: left; align-items: flex-start; }
    h1 { color: var(--accent-blue-light); font-weight: 600; font-size: 1.5em; border-bottom: 2px solid var(--accent-blue); padding-bottom: 6px; margin-bottom: 16px; }
    h2 { color: var(--accent-green); font-size: 1.25em; }
    h3 { color: var(--accent-yellow); font-size: 1.05em; }
    h6 { color: var(--fg-secondary); font-weight: 400; font-size: 0.8em; }
    strong { color: var(--accent-orange); }
    table { width: 100%; border-collapse: collapse; font-size: 0.75em; border: 1px solid var(--border); }
    th { background: var(--bg-elevated); color: var(--accent-blue-light); padding: 8px 12px; text-align: left; font-weight: 600; border-bottom: 2px solid var(--accent-blue); }
    td { padding: 6px 12px; border-bottom: 1px solid var(--border); }
    tr:nth-child(even) { background: var(--bg-secondary); }
    code { font-family: 'Fira Code', monospace; background: var(--bg-secondary); color: var(--accent-orange); padding: 2px 6px; border-radius: 4px; border: 1px solid var(--border); }
    pre { background: var(--bg-secondary); padding: 12px 16px; border-radius: 6px; font-family: 'Fira Code', monospace; font-size: 0.65em; line-height: 1.4; border: 1px solid var(--border); border-left: 3px solid var(--accent-blue); }
    pre code { background: transparent; border: none; padding: 0; color: var(--fg-primary); }
    blockquote { border-left: 4px solid var(--accent-blue); background: var(--bg-secondary); padding: 10px 16px; font-style: italic; border-radius: 0 6px 6px 0; }
    blockquote strong { color: var(--accent-green); }
    ul, ol { font-size: 0.95em; line-height: 1.5; margin: 8px 0; }
    li { margin-bottom: 4px; }
    li::marker { color: var(--accent-blue); }
    section.title { background: linear-gradient(135deg, var(--bg-primary) 0%, #0d1117 100%); text-align: center; justify-content: center; }
    section.title h2 { color: var(--accent-blue-light); font-size: 1.8em; border: none; }
    section.title h2:nth-of-type(2) { color: var(--accent-green); font-size: 1.3em; }
    section.dense { font-size: 16px; padding: 30px 40px; }
    section.dense table { font-size: 0.85em; }
    section::after { color: var(--fg-secondary); }

---

## cobol2java

## Implementation Plan

###### Chandra Mohn -- 2026-03-21

---

# Architecture: What We Reuse

```
COBOL Source
    |
    v
+----------------------------------+
| COBOL Parser + AST               |  100% REUSE (zero changes)
| Preprocessor (COPY/REPLACE)      |
| Statement Coverage Analysis      |
| Field Access Analysis            |
| PIC Clause / Type Analysis       |
+----------------------------------+
    |
    |--- --target rust ---> [Rust Codegen]   (existing, untouched)
    |
    |--- --target java ---> [Java Codegen]   (NEW)
    |
    v
+----------------------------------+
| Nexflow DSL Emitters             |  100% REUSE (reads attributes)
| CLI (scan, check, parse, diff)   |  100% REUSE
+----------------------------------+
```

> The parser, AST, analysis, CLI, and DSL emitters are **shared**. Only the codegen backend is target-specific.

---

# What Changes

| Component | Effort | Description |
|---|---|---|
| `codegen_java/` module | **Large** | 6 new files parallel to Rust codegen |
| `cobol-java-runtime` | **Medium** | Java library: CobolDecimal, CobolString, CobolFile |
| `JavaWriter` | **Small** | Output buffer (same pattern as RustWriter) |
| `--target java` CLI flag | **Trivial** | Route to Java codegen in transpile command |
| Java `@Cobol(...)` annotations | **Small** | Java equivalent of `#[cobol(...)]` attributes |
| Java Maven project template | **Small** | pom.xml with runtime dependency |

### What Does NOT Change

- COBOL parser, preprocessor, AST
- Field analysis, condition maps, record maps
- Nexflow DSL emitters (they read annotations, same concept in Java)
- CLI subcommands (scan, check, parse, preprocess, diff)
- Rust codegen (completely independent)

---

# Code Mapping: Rust vs Java

| COBOL Construct | Rust Output | Java Output |
|---|---|---|
| WORKING-STORAGE | `pub struct WorkingStorage { ... }` | `public class WorkingStorage { ... }` |
| PIC 9(5) | `PackedDecimal::new(5, 0, false)` | `new CobolDecimal(5, 0, false)` |
| PIC X(10) | `PicX::new(10, b"value")` | `new CobolString(10, "value")` |
| PIC S9(9)V99 COMP-3 | `PackedDecimal::new(9, 2, true)` | `new CobolDecimal(9, 2, true)` |
| Paragraph | `fn para_name(ws: &mut WS, ctx: &mut Ctx)` | `public void paraName(WS ws, Ctx ctx)` |
| IF/ELSE | `if condition { ... } else { ... }` | `if (condition) { ... } else { ... }` |
| EVALUATE | `if ... else if ... else` | `if ... else if ... else` (or `switch`) |
| MOVE alpha | `move_alphanumeric_literal(b"X", &mut f, cfg)` | `CobolRuntime.moveAlpha("X", f)` |
| MOVE numeric | `move_numeric_literal(d, &mut f, cfg)` | `CobolRuntime.moveNumeric(d, f)` |
| ADD | `cobol_add(&op, &mut tgt, rnd, cfg)` | `CobolRuntime.add(op, tgt)` |
| COMPUTE | `cobol_compute(expr, &mut tgt, rnd, cfg)` | `CobolRuntime.compute(expr, tgt)` |
| PERFORM | `target_para(ws, ctx)` | `targetPara(ws, ctx)` |
| OPEN/READ/WRITE | `ws.file.open(mode)` | `ws.file.open(mode)` |
| DISPLAY | `print!("{}", field.display_bytes())` | `System.out.println(field.display())` |
| EXEC SQL | `sql.exec_query(...)` | `sql.execQuery(...)` |

---

# Java Runtime Library

### `cobol-java-runtime` (Maven artifact)

```
com.mphasis.cobol.runtime/
    CobolDecimal.java       -- PackedDecimal equivalent (wraps BigDecimal)
    CobolString.java        -- PicX equivalent (fixed-length byte array)
    CobolRuntime.java       -- Static methods: move, add, subtract, compute
    CobolFile.java          -- Sequential/Relative/Indexed file abstraction
    CobolSqlRuntime.java    -- SQL interface (JDBC-based)
    RuntimeConfig.java      -- Decimal precision, truncation rules
    ProgramContext.java     -- Return code, stop flag, goto target
```

| Rust Type | Java Equivalent |
|---|---|
| `PackedDecimal` | `CobolDecimal` (wraps `BigDecimal`) |
| `PicX` | `CobolString` (wraps `byte[]`) |
| `Decimal` (rust_decimal) | `BigDecimal` (java.math) |
| `SequentialFile` | `CobolFile<Sequential>` |
| `RuntimeConfig` | `RuntimeConfig` (same concept) |
| `dyn CobolSqlRuntime` | `CobolSqlRuntime` interface (JDBC impl) |

> Java has native `BigDecimal` -- COBOL's decimal arithmetic maps naturally.

---

<!-- _class: dense -->

# Java Codegen Module Structure

```
crates/cobol-transpiler/src/
    codegen/
        mod.rs              -- existing, add java module export
        rust_writer.rs      -- existing (untouched)
        attributes.rs       -- existing (untouched)
        data_gen.rs         -- existing (untouched)
        field_analysis.rs   -- existing (untouched)
        proc_gen.rs         -- existing (untouched)
        java/
            mod.rs          -- Java codegen entry point
            java_writer.rs  -- JavaWriter (indentation + braces)
            data_gen.rs     -- WorkingStorage class generation
            proc_gen.rs     -- Paragraph methods + statement generation
            attributes.rs   -- @Cobol(...) annotation generation
            expr_gen.rs     -- Operand/condition/arithmetic expressions
```

### Key Design Decision

**Parallel modules, shared AST.** The Java codegen reads the same `CobolProgram` AST as the Rust codegen. No intermediate representation. Each target has its own codegen functions that produce target-specific syntax.

---

# Implementation Sessions

| Session | What | Deliverable |
|---|---|---|
| **J1** | JavaWriter + data_gen | WorkingStorage class with fields + constructor |
| **J2** | proc_gen (control flow) | IF, EVALUATE, PERFORM, run() dispatch |
| **J3** | proc_gen (statements) | MOVE, COMPUTE, ADD/SUB/MUL/DIV, DISPLAY |
| **J4** | File I/O + SQL | OPEN/READ/WRITE/CLOSE, EXEC SQL (JDBC) |
| **J5** | Java runtime library | CobolDecimal, CobolString, CobolRuntime |
| **J6** | CLI + Maven template | --target java, pom.xml, E2E test |
| **J7** | Stress tests | Run 47 stress tests against Java output |
| **J8** | @Cobol annotations | Java annotations for DSL emitter compatibility |

### Reuse Estimate

| Component | Lines (Rust) | Lines (Java, est.) | Reuse |
|---|---|---|---|
| Parser + AST | ~15,000 | 0 | 100% |
| Field analysis | 732 | 0 | 100% |
| Attributes | 349 | ~200 | Pattern reuse |
| Data gen | 1,692 | ~1,200 | Logic reuse, syntax change |
| Proc gen | 5,030 | ~4,000 | Logic reuse, syntax change |
| **Total new code** | | **~5,400** | |

---

# Java Output Example

### COBOL Source
```cobol
01  WS-SCORE    PIC 9(3) VALUE 750.
01  WS-RESULT   PIC X(10).

EVALUATE TRUE
    WHEN WS-SCORE > 750  MOVE "EXCELLENT" TO WS-RESULT
    WHEN WS-SCORE > 650  MOVE "GOOD" TO WS-RESULT
    WHEN OTHER           MOVE "FAIR" TO WS-RESULT
END-EVALUATE.
```

### Java Output
```java
@Cobol(program = "RATECHECK")
public class WorkingStorage {
    @Cobol(level = 1, pic = "9(3)")
    public CobolDecimal wsScore = new CobolDecimal(3, 0, false).set(750);

    @Cobol(level = 1, pic = "X(10)")
    public CobolString wsResult = new CobolString(10);
}

@Cobol(section = "MAIN-SECTION", reads = "WS-SCORE", writes = "WS-RESULT")
public void mainPara(WorkingStorage ws, ProgramContext ctx) {
    if (ws.wsScore.compareTo(750) > 0) {
        CobolRuntime.moveAlpha("EXCELLENT", ws.wsResult);
    } else if (ws.wsScore.compareTo(650) > 0) {
        CobolRuntime.moveAlpha("GOOD", ws.wsResult);
    } else {
        CobolRuntime.moveAlpha("FAIR", ws.wsResult);
    }
}
```

---

# DSL Emitter Compatibility

The Nexflow DSL emitters currently read `#[cobol(...)]` Rust attributes via `syn`.
For Java, two options:

### Option A: Read Java `@Cobol(...)` annotations (new parser)

Requires a Java annotation parser in Rust. Medium effort.

### Option B: Emit DSL from AST directly (bypass target code)

```
COBOL AST ---+---> Rust codegen  ---> .rs files
             |
             +---> Java codegen  ---> .java files
             |
             +---> DSL emitters  ---> .schema/.rules/.xform/.proc
```

The DSL emitters read the **COBOL AST directly** instead of reading
generated Rust/Java. This is cleaner -- the DSL represents COBOL
business logic, which is the same regardless of target language.

> **Recommendation: Option B.** Refactor DSL emitters to read COBOL AST.
> This makes DSL emission target-independent and eliminates the dependency
> on syn/Rust attributes entirely.

---

# Risk Assessment

| Risk | Impact | Mitigation |
|---|---|---|
| Java BigDecimal precision differs from Rust Decimal | **Medium** -- edge case arithmetic | Test with COBOL stress tests, compare outputs |
| Java lacks Rust's ownership model | **Low** -- COBOL is mutable by nature | Java's mutable objects map naturally |
| Java String vs byte[] for PIC X | **Low** -- well-understood mapping | CobolString wraps byte[] like PicX |
| Java file I/O differs from Rust | **Medium** -- BufferedReader vs BufReader | Thin abstraction in runtime library |
| EXEC SQL: JDBC vs rusqlite/duckdb | **Medium** -- different APIs | CobolSqlRuntime interface abstracts it |
| Performance: Java GC vs Rust zero-cost | **Low** -- correctness first | Optimize later if needed |

---

# Timeline

```
Week 1:  J1-J2  JavaWriter, WorkingStorage, IF/EVALUATE/PERFORM
Week 2:  J3-J4  Statements (MOVE, arithmetic, file I/O, SQL)
Week 3:  J5-J6  Java runtime library, CLI --target java, Maven template
Week 4:  J7-J8  Stress tests, @Cobol annotations, E2E validation
```

### Success Criteria

- [ ] `cobol2rust transpile --target java program.cbl` produces compilable Java
- [ ] Java output passes the same 47 stress tests as Rust output
- [ ] `cobol2rust rustify --emit-dsl` works on both Rust and Java output
- [ ] Maven project builds and runs with `mvn compile exec:java`
- [ ] Rust target completely unaffected (zero changes to existing codegen)
