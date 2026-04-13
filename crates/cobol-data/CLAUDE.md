# cobol-data

COBOL binary data intelligence: layout, decode, REDEFINES, discriminators, discovery, viewer.

## Architecture

```
cobol-transpiler (parser, AST)
  |
  v
[cobol-data]
  encoding.rs      CP037 EBCDIC/ASCII, auto-detect
  layout.rs        Byte offset computation on DataEntry trees
  decode.rs        Field decode (zoned, packed, binary, text)
  record.rs        Record-level decode with variant selection
  redefines.rs     REDEFINES group types + extraction
  discriminator.rs Discriminator detection from ProcedureDivision AST
  discovery.rs     File-to-copybook matching (program link + length)
  session.rs       ViewerSession (stateful, windowed decode)
  export.rs        JSON/CSV export
  error.rs         Error types
```

## Key Design

- No parser -- uses cobol-transpiler's AST (DataEntry, ProcedureDivision)
- Discriminator detection walks the typed AST (IF, EVALUATE, 88-level)
- ViewerSession<F: FileAccess> is generic over file backend (native/WASM)
- Replaces coqu-di (~/workspace/coqu/coqu-di/) which is now reference-only

## Dependencies

cobol-transpiler, serde, serde_json, thiserror, miette

## CLI Integration

Three subcommands in cobol-cli:
- `cobol2rust decode` -- decode binary dataset to JSON/CSV
- `cobol2rust discover` -- auto-match data files to copybooks
- `cobol2rust data-analyze` -- show layout, REDEFINES, discriminators

## Tests

96 inline tests across all modules.
