# cobol-io

COBOL file I/O: sequential, indexed (SQLite-backed), relative, print files.

## Key Types

- `CobolFile` trait -- Abstract file interface (OPEN, CLOSE, READ, WRITE, REWRITE, DELETE)
- `SequentialFile` -- READ/WRITE in order
- `IndexedFile` -- Keyed access via SQLite B-tree indexes (ISAM semantics)
- `RelativeFile` -- Record-number-based access
- `FileResolver` -- Filename resolution and file operations
- `FileStatusCode` -- COBOL file status codes (00, 10, 21, 23, 35, etc.)

## Modules

- `file_traits.rs` -- CobolFile trait, FileOpenMode, FileAccessMode, FileOrganization
- `file_status.rs` -- FileStatusCode enum (maps to COBOL 2-digit status)
- `sequential.rs` -- Sequential file implementation
- `indexed.rs` -- Indexed file (SQLite backend)
- `relative.rs` -- Relative file implementation
- `resolver.rs` -- File path resolution

## Dependencies

cobol-core, cobol-types, cobol-move, rusqlite (optional, for indexed files)

## Features

- `sqlite` (default) -- Indexed file support via SQLite
- Without `sqlite` -- Sequential and relative only

## Rules

- FileStatusCode must match IBM COBOL semantics exactly
- Indexed files use SQLite for B-tree indexing (ISAM replacement)
- OPEN modes: INPUT, OUTPUT, I-O, EXTEND
- READ/WRITE operate on record buffers (byte arrays)
