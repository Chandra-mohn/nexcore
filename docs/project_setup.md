 Project Config: scripts/project-ssupdate/

  Two files:

  .cobol2rust.toml -- project configuration

  Drop this in the client project root (alongside ./cobol/). It tells nexmig where copybooks live and
  how to run:

  [workspace]
  copy_paths = [
      "cobol/copybook/code-copybooks",
      "cobol/copybook/layout-copybooks",
  ]

  [pipeline]
  output = "rust-out"
  jobs = 2
  continue_on_error = true
  incremental = true

  run_pipeline.sh -- full transpile + DSL pipeline

  # On the client machine:
  cd /path/to/client-repo
  cp /path/to/nexcore/scripts/project-ssupdate/.cobol2rust.toml .

  # Run full pipeline (direct emit by default)
  bash /path/to/nexcore/scripts/project-ssupdate/run_pipeline.sh

  # Or override settings:
  EMIT_MODE=compare JOBS=1 bash run_pipeline.sh

  The pipeline runs two phases:

  1. Transpile -- nexmig transpile ./cobol/cobolfile --workspace --parallel -j 2 (reads .cobol2rust.toml
   for copybooks automatically)
  2. DSL Emit -- nexmig rustify ./rust-out --emit-dsl --emit-mode direct --cobol-source
  ./cobol/cobolfile -j 2

  Output lands in:
  ./rust-out/
    programs/ssupdate/src/main.rs   (271K lines)
    dsl/
      schema/*.schema
      transform/*.xform
      rules/*.rules
      process/*.proc
      dsl_manifest.json

  The script has a commented-out Mode B that also applies Tier 1 rustify transforms before DSL emission
  (--tier 1 -o ./rustified), if you want both idiomatic Rust and DSL output.

