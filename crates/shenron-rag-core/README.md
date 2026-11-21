# shenron-rag-core

Pure, wasm32-safe core of the Shenron RAG layer: source-aware chunkers (Rust, Markdown, structural, prose), reciprocal-rank fusion, MMR and vector helpers, and a port of shenron's retrieval eval harness checked against golden files (`tests/fixtures/parity/`).

## Development

```sh
cargo test -p shenron-rag-core
```

Builds run on the build host or in CI; workspace rules are in the root [README](../../README.md).
