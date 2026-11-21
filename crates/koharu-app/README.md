# koharu-app

GPUI Kit workspace of the Koharu editing engine (`KoharuWorkspace`), without a Tauri IPC layer.

## Development

```sh
cargo check -p koharu-app
cargo nextest run -p koharu-app
```

Builds run on the build host; workspace rules are in the root [README](../../README.md).
