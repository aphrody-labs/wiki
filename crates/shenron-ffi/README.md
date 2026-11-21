# shenron-ffi

Shenron C ABI (`shenron_*`): the `shenron` command, RAG chunking and search, persona, local corpus and cancellable jobs.

## Development

```sh
cargo check -p shenron-ffi
cargo nextest run -p shenron-ffi
```

Builds run on the build host; workspace rules are in the root [README](../../README.md).
