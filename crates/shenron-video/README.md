# shenron-video

Rust video library without ffmpeg: MPEG-1/2 decoding, H.264 encoding, fragmented MP4 muxing, probing and still images. Excluded from the Cargo workspace (`exclude` in `Cargo.toml`).

## Development

```sh
cd crates/shenron-video   # excluded from the workspace: own Cargo.lock
cargo check
cargo nextest run
```

Builds run on the build host; workspace rules are in the root [README](../../README.md).
