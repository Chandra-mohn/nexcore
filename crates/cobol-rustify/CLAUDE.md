# cobol-rustify

Phase 2 rustification: transform COBOL-shaped Rust into idiomatic Rust.
Also contains the Nexflow DSL emitters.

## Architecture

Tiered, idempotent, non-destructive promotion pipeline.

```
Phase 1 Rust (from transpiler)
  -> [Tier 1] Cosmetic cleanup
  -> [Tier 2] Type promotion (safety-gated)
  -> [Tier 3] Structural assessment (plan only)
  -> [Tier 4] Structural transformation (config-driven)
  -> Phase 2 Rust (idiomatic)
```

## Tiers

| Tier | Type | Rules |
|------|------|-------|
| T1 | Cosmetic | const-extract, zero-literal, dead-field, allow-cleanup, unused-import, display-simplify |
| T2 | Type Promotion | pic-to-string, packed-to-native, localize-vars, bool-extract, enum-extract |
| T3 | Assessment | dispatcher-analysis, ws-decomposition, status-to-result, io-modernize |
| T4 | Structural | dispatch, domain, error-type, io-backend, structural |

## Source Structure

```
src/
  rules/
    tier1/   -- 6 rules (cosmetic)
    tier2/   -- 5 rules (type promotion)
    tier3/   -- 4 rules (assessment)
    tier4/   -- 5 rules (structural)
    mod.rs   -- RuleRegistry, rule dispatch
  dsl/
    schema_emitter.rs    -- E1: PIC->typed fields, entity decomposition
    transform_emitter.rs -- E2: function classification, IoSpec
    rules_emitter.rs     -- E3: decision tables from match/if
    process_emitter.rs   -- E4: call graph, entry points, cycle detection
    expr_extract.rs      -- Expression extraction from Rust AST
    dsl_ast.rs           -- Typed DSL AST (grammar-valid by construction)
    type_mapping.rs      -- COBOL -> Nexflow type mapping
    writer.rs            -- emit_all_dsl(), write_dsl_files()
    cobol_attrs.rs       -- #[cobol(...)] attribute reading
    integration_test.rs  -- E2E tests
  config.rs, error.rs, hints.rs, manifest.rs
  patch.rs, plan.rs, review.rs, safety.rs
  target_config.rs
```

## Key Functions

- `analyze_workspace()` -- Read Rust source, apply rules, return AnalysisReport
- `apply_workspace()` -- Copy source, apply transforms, write manifest + review
- `emit_dsl_for_workspace()` -- Generate Nexflow DSL files

## DSL Emitter Notes

- 4 emitters: Schema (E1), Transform (E2), Rules (E3), Process (E4)
- Typed AST: confidence 1.00 (grammar-valid by construction)
- DSL emission allowed on Phase 1 Rust (schemas, rules, process are complete)
- Transform expressions deferred to Stage 2 (after Tier 2 rustification)
- 40/60 split: 40% from COBOL (business logic), 60% from Nexflow (architecture)

## Outputs

- rustify/ directory: manifest.json, hints.json, review.md, plan.md
- dsl/: .schema, .rules, .xform, .proc files + dsl_manifest.json

## Testing

- 202 tests total (inline in modules + dsl/integration_test.rs)
- Tests verify idempotency, patch detection, safety gates
- DSL tests: 20 schema + 13 xform + 13 types + 11 rules + 8 process + 17 integration

## Dependencies

syn, quote, proc-macro2, serde_json, rayon, thiserror

## Rules

- Rules are idempotent: running twice produces same output
- Manifests track SHA2 checksums for reproducibility
- User patches are detected and preserved
- Target config: .cobol2rust-target.toml controls T4 behavior
