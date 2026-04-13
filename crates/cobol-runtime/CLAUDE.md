# cobol-runtime

COBOL program runtime: lifecycle, prelude re-exports, special registers, built-in verbs.
This is what generated COBOL programs link against.

## Key Modules

- `prelude` -- Re-exports from all runtime crates (the one import generated code needs)
- `program` -- Program lifecycle (ENTRY, EXIT, STOP RUN)
- `special_regs` -- RETURN-CODE, WS-CALL-COUNT, etc.
- `call` -- CALL statement runtime
- `display` -- DISPLAY verb
- `perform_stack` -- PERFORM ... UNTIL context stack
- `inspect` -- INSPECT verb (TALLYING, REPLACING, CONVERTING)
- `string_verb` / `unstring_verb` -- STRING, UNSTRING verbs
- `ref_mod` -- Reference modification (substring)
- `intrinsics` -- Built-in functions (LENGTH, REVERSE, UPPER-CASE, etc.)

## Dependencies

cobol-core, cobol-types, cobol-move, cobol-arithmetic
Optional: cobol-io, cobol-sort, cobol-sql

## Features

- `full` (default) -- All subsystems (io, sort, sql)
- `minimal` -- Types + move + arithmetic only
- `io`, `sort`, `sql` -- Individual subsystems

## Rules

- Generated code does `use cobol_runtime::prelude::*;`
- The prelude re-exports everything the generated code needs
- Keep this crate's own code minimal -- it's mostly orchestration and re-exports
- Feature flags control which subsystems are compiled in
