# Tier 4 Rules Spec: Structural Transformation

## Overview

Tier 4 rules perform deterministic structural transformations consuming:
1. **Target config** (`.cobol2rust-target.toml`) -- architectural choices
2. **Tier 3 assessment** (`.rustify/plan.md` / assessment transforms) -- detected patterns
3. **COBOL hints** (`.rustify/hints.json`) -- semantic metadata
4. **Source Rust code** -- the workspace to transform

Unlike Tiers 1-3 (per-file analysis), Tier 4 operates at **workspace level**:
- May restructure multiple files simultaneously
- May create new files (structs, enums, service modules)
- May delete or merge files
- Output is a new workspace directory (same as lower tiers)

## Architecture: Why a New Trait

The existing `RustifyRule::analyze(&self, ctx: &AnalysisContext) -> Vec<Transform>` is per-file.
Tier 4 needs:
- Access to the target config
- Access to ALL files simultaneously (cross-file transforms)
- Ability to create new files, not just edit existing ones
- Awareness of what Tier 3 detected (assessment descriptions)

### New Trait: `StructuralRule`

```rust
pub trait StructuralRule: Send + Sync {
    fn id(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn analyze(&self, ctx: &StructuralContext) -> StructuralPlan;
}
```

Where `StructuralContext` provides:

```rust
pub struct StructuralContext<'a> {
    /// All source files keyed by relative path.
    pub files: &'a HashMap<String, SourceFile>,
    /// COBOL semantic hints.
    pub hints: Option<&'a HintsFile>,
    /// Target architecture config.
    pub target: &'a TargetConfig,
    /// Tier 3 assessment results (from previous run).
    pub assessments: &'a [Transform],
}

pub struct SourceFile {
    pub path: PathBuf,
    pub text: String,
    pub parsed: syn::File,
}
```

And `StructuralPlan` captures cross-file operations:

```rust
pub struct StructuralPlan {
    pub rule_id: String,
    pub description: String,
    /// Files to modify (relative path -> new content).
    pub modified_files: HashMap<String, String>,
    /// New files to create (relative path -> content).
    pub new_files: HashMap<String, String>,
    /// Files to delete (relative paths).
    pub deleted_files: Vec<String>,
    /// Cargo.toml modifications (dependency additions, etc.).
    pub cargo_edits: Vec<CargoEdit>,
    /// Human-readable summary for review.
    pub summary: String,
}
```

## Four Tier 4 Rules

### T4-DISPATCH: Dispatcher Elimination

**Consumes**: `[control_flow]` config + Tier 3 `dispatcher-analysis` assessment

**Strategies**:

#### strategy = "inline"
- Read Tier 3 assessment to get leaf paragraphs and their body sizes
- For each leaf paragraph with body_lines <= inline_threshold:
  - Find all call sites in the dispatch table
  - Replace call with inlined body
  - Remove the standalone function
- If collapse_chains = true:
  - Detect A -> B -> C chains (each calls exactly one other)
  - Merge into single function with combined body
- After inlining all eligible leaves:
  - If only 1 paragraph remains in dispatch: remove dispatch loop entirely
  - If dispatch still has entries: keep as simplified match

#### strategy = "functions"
- Remove the dispatch loop entirely
- Replace with sequential function calls in `run()`:
  ```rust
  fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
      init_100(ws, ctx);
      process_200(ws, ctx);
      cleanup_300(ws, ctx);
  }
  ```
- If error_propagation = "result":
  - Change paragraph signatures to `fn name(...) -> Result<(), ProgramError>`
  - Add `?` at each call site
  - Generate `ProgramError` enum in the file

#### strategy = "state-machine"
- Replace `_pc: usize` with typed enum:
  ```rust
  enum ProgramState {
      Init100,
      Process200,
      Cleanup300,
      Done,
  }
  ```
- Replace `match _pc { 0 => ... }` with `match state { ProgramState::Init100 => ... }`
- State transitions become `state = ProgramState::Process200` instead of `_pc = 1`

#### strategy = "async"
- Make all paragraph functions `async fn`
- Replace dispatch loop with sequential `.await` calls
- Wrap in `#[tokio::main]` or runtime block

#### cycle_handling = "trampoline"
- For detected cycles (from Tier 3 assessment):
  ```rust
  enum Trampoline {
      ParaA,
      ParaB,
      Done,
  }
  fn run_cycle(ws: &mut WS) {
      let mut state = Trampoline::ParaA;
      loop {
          state = match state {
              Trampoline::ParaA => para_a_body(ws),
              Trampoline::ParaB => para_b_body(ws),
              Trampoline::Done => break,
          };
      }
  }
  ```

### T4-DOMAIN: WorkingStorage Decomposition

**Consumes**: `[data_model]` config + Tier 3 `ws-decomposition` assessment + hints

**Strategies**:

#### strategy = "flat" + naming = "strip-prefix"
- Simple field rename: `ws.ws_acct_number` -> `ws.acct_number`
- Regex replace across all files in workspace
- Update struct declaration

#### strategy = "nested"
- Read Tier 3 assessment for proposed sub-structs (prefix groups + co-access groups)
- Or use `data_model.structs` custom overrides if provided
- For each proposed sub-struct:
  1. Generate struct definition with derive_level traits
  2. Replace fields in WorkingStorage with the sub-struct:
     ```rust
     // Before:
     pub ws_acct_number: PicX,
     pub ws_acct_type: PicX,
     pub ws_acct_balance: PackedDecimal,
     // After:
     pub acct: AcctInfo,
     ```
  3. Rewrite all field accesses: `ws.ws_acct_number` -> `ws.acct.number`
  4. If naming = "strip-prefix": strip the common prefix from sub-struct fields
  5. If use_accessors = true: generate `impl AcctInfo { pub fn number(&self) -> &PicX { ... } }`

#### strategy = "split"
- Uses co-access groups from Tier 3 assessment
- Each group becomes a separate struct parameter:
  ```rust
  // Before: fn process(ws: &mut WorkingStorage)
  // After:  fn process(acct: &mut AcctInfo, txn: &mut TxnData)
  ```
- Requires rewriting all function signatures and call sites

#### strategy = "domain"
- Most aggressive: combines nested + type promotion from Tier 2
- Sub-struct fields get promoted types (String instead of PicX, Decimal instead of PackedDecimal)
- Requires all Tier 2 safety gates to pass for included fields

### T4-ERROR: Status Code to Error Type

**Consumes**: `[error_handling]` config + Tier 3 `status-to-result` assessment

#### strategy = "result"
- For each status field detected by Tier 3:
  - Read values_set and check_count from assessment
  - If binary_status applies (2 values):
    - Generate `Result<(), ProgramError>` return type
    - Replace `.pack(Decimal::from(0))` with `Ok(())`
    - Replace `.pack(Decimal::from(1))` with `Err(ProgramError::...)`
    - Replace `.to_decimal() == Decimal::from(0)` with `.is_ok()`
  - If multi_status applies (3+ values):
    - Generate status enum with variants from values
    - Use custom mappings if `error_handling.mappings` has entries for this field
    - Replace pack/check patterns with enum construction/matching

#### strategy = "thiserror"
- Same as "result" but generated error types use `#[derive(thiserror::Error)]`
- Add `thiserror` to Cargo.toml dependencies

#### generate_program_error = true
- Collect all status field errors into one top-level enum:
  ```rust
  #[derive(Debug, thiserror::Error)]
  pub enum ProgramError {
      #[error("account validation failed: {0}")]
      AccountValidation(AccountStatus),
      #[error("I/O error: {0}")]
      IoError(FileStatus),
  }
  ```

### T4-IO: I/O Modernization

**Consumes**: `[io]` config + Tier 3 `io-modernize` assessment

#### backend = "file" (default)
- Use Tier 3 access pattern classification:
  - ReadSequential -> `BufReader::new(File::open(...)?).lines()`
  - WriteSequential -> `BufWriter::new(File::create(...)?)` with `writeln!`
  - ReadWrite -> `OpenOptions::new().read(true).write(true)`
  - Indexed -> `HashMap<Key, Record>` loaded from file
- Replace `ws.handle.open_input(...)` with Rust std::fs calls
- Replace `ws.handle.read_next(...)` with iterator patterns
- Replace `ws.handle.write_record(...)` with `write!`/`writeln!`
- Remove handle fields from WorkingStorage

#### backend = "database"
- Per-handle table name from `io.handles.X.table`
- Generate struct matching the record layout:
  ```rust
  #[derive(Debug, sqlx::FromRow)]
  struct MasterRecord {
      acct_number: String,
      acct_type: String,
      balance: Decimal,
  }
  ```
- Replace open -> connect, read_next -> SELECT query, write_record -> INSERT
- Add database connection to function parameters
- Add sqlx/rusqlite to Cargo.toml

#### backend = "stream"
- Per-handle topic from `io.handles.X.topic`
- ReadSequential -> consumer loop:
  ```rust
  let consumer = StreamConsumer::new(config)?;
  consumer.subscribe(&["topic"])?;
  for message in consumer.iter() { ... }
  ```
- WriteSequential -> producer:
  ```rust
  let producer = FutureProducer::new(config)?;
  producer.send(record).await?;
  ```
- Add rdkafka/async-nats to Cargo.toml

#### backend = "api"
- Per-handle endpoint from `io.handles.X.endpoint`
- Generate HTTP client calls using reqwest
- ReadSequential -> GET with pagination
- WriteSequential -> POST per record

## Execution Order

Tier 4 rules must execute in dependency order:

```
T4-DISPATCH (1st) -- restructures control flow
    |
    v
T4-DOMAIN (2nd) -- restructures data (needs stable function signatures)
    |
    v
T4-ERROR (3rd) -- adds error types (needs stable struct layout)
    |
    v
T4-IO (4th) -- modernizes I/O (needs stable data + error types)
```

Each rule produces a `StructuralPlan`. Plans are applied sequentially -- each
subsequent rule sees the output of the previous one.

## Integration with Existing Pipeline

### Tier Enum Extension

```rust
pub enum Tier {
    Tier1 = 1,  // Cosmetic
    Tier2 = 2,  // Type promotion
    Tier3 = 3,  // Structural assessment
    Tier4 = 4,  // Structural transformation
}
```

### Workflow

```
cobol2rust rustify source/ --tier 4 --output transformed/
```

This runs Tiers 1-3 first (cumulative), then applies Tier 4 structural rules.
Tier 4 requires `.cobol2rust-target.toml` in the source directory.

### apply_workspace Changes

The `apply_workspace` function in `lib.rs` gains a new step:

```rust
// After Step 7 (plan.md):
if config.tier >= Tier::Tier4 {
    let target_config = target_config::load_target_config(&source_dir)?;
    let structural_rules = tier4::tier4_rules();
    let mut workspace = load_workspace_files(&output_dir)?;

    for rule in &structural_rules {
        let ctx = StructuralContext {
            files: &workspace,
            hints: hints.as_ref(),
            target: &target_config,
            assessments: &all_transforms,
        };
        let plan = rule.analyze(&ctx);
        apply_structural_plan(&mut workspace, &plan)?;
    }

    write_workspace_files(&output_dir, &workspace)?;
}
```

## File Generation Patterns

### New Struct File (T4-DOMAIN)
When extracting sub-structs, generate `src/types/acct_info.rs`:
```rust
//! Generated by cobol2rust rustify T4-DOMAIN.
//! Source: WorkingStorage fields with prefix ws_acct_

use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct AcctInfo {
    pub number: String,
    pub acct_type: String,
    pub balance: Decimal,
}

impl Default for AcctInfo {
    fn default() -> Self {
        Self {
            number: String::new(),
            acct_type: String::new(),
            balance: Decimal::ZERO,
        }
    }
}
```

### Error Enum File (T4-ERROR)
Generate `src/error.rs`:
```rust
//! Generated by cobol2rust rustify T4-ERROR.

#[derive(Debug, thiserror::Error)]
pub enum ProgramError {
    #[error("validation status: {0}")]
    ValidationStatus(ValidationStatus),
    #[error("I/O error: {0}")]
    IoStatus(std::io::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationStatus {
    Success,
    InvalidInput,
    NotFound,
}
```

### Cargo.toml Edits (T4-IO, T4-ERROR)
```rust
pub struct CargoEdit {
    /// Dependency to add (e.g., "thiserror").
    pub dependency: String,
    /// Version requirement (e.g., "1.0").
    pub version: String,
    /// Optional features (e.g., ["runtime-tokio", "macros"]).
    pub features: Vec<String>,
}
```

## Implementation Sessions

### Session B1: Infrastructure (~2 hours)
- Add `Tier::Tier4` to enum
- Create `StructuralRule` trait, `StructuralContext`, `StructuralPlan`
- Create `tier4/mod.rs` with rule registry
- Wire into `apply_workspace` conditional on tier >= 4
- Unit tests for plan application

### Session B2: T4-DISPATCH (~3 hours)
- Implement all 4 strategies (inline, functions, state-machine, async)
- Implement cycle_handling (preserve, trampoline, recursive)
- Implement collapse_chains
- Implement error_propagation (none, result, anyhow)
- Tests for each strategy on synthetic dispatch loops

### Session B3: T4-DOMAIN (~3 hours)
- Implement all 4 strategies (flat, nested, split, domain)
- Implement naming strategies (preserve, strip-prefix, rust-idiom)
- Implement derive_level and use_accessors
- Cross-file field access rewriting
- Tests for sub-struct extraction and field rename propagation

### Session B4: T4-ERROR (~2 hours)
- Implement strategies (status-quo, result, thiserror, anyhow)
- Implement binary_status and multi_status targets
- Implement custom mappings
- Generate error enum files
- Tests for status field -> Result conversion

### Session B5: T4-IO (~3 hours)
- Implement backends (file, database, stream, api)
- Implement per-handle overrides
- Implement format handling
- Generate database struct files
- Cargo.toml dependency injection
- Tests for each backend pattern

### Session B6: Integration Testing (~2 hours)
- End-to-end: transpiled COBOL -> Tier 1-4 -> compilable Rust
- Test with different target config presets
- Test per-program overrides
- Verify idempotency

## Constraints

1. **Generated code must compile**: Every T4 rule output must be `cargo check` clean
2. **Behavioral equivalence**: Same inputs -> same outputs as the pre-T4 code
3. **Incremental**: Each rule's output is independently useful
4. **Idempotent**: Running T4 twice with same config produces same output
5. **Config-driven**: No hardcoded architectural choices -- everything from .toml
6. **Reviewable**: StructuralPlan has human-readable summary for engineer review
