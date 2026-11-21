# shenron-cli

The `shenron` command: health, RAG, training, the databooks OCR bridge, the dragonballfr.com databooks API, manga and configuration.

## Development

```sh
cargo check -p shenron-cli
cargo nextest run -p shenron-cli
```

Builds run on the build host; workspace rules are in the root [README](../../README.md).
