# koharu-ml

Koharu models and inference: text and speech-bubble detection, OCR, inpainting and page layout, with benchmarks.

## Development

```sh
cargo check -p koharu-ml
cargo nextest run -p koharu-ml
```

Builds run on the build host; workspace rules are in the root [README](../../README.md).
