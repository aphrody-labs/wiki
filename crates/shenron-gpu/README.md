# shenron-gpu

The workspace's single wgpu backend: adapter and device selection, limits, device loss, render targets, readback and presentation.

## Development

```sh
cargo check -p shenron-gpu
cargo nextest run -p shenron-gpu
```

Builds run on the build host; workspace rules are in the root [README](../../README.md).
