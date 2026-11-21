# koharu-agent

In-process Koharu Codex agent, authenticated through OAuth.

## Development

```sh
cargo check -p koharu-agent
cargo nextest run -p koharu-agent
```

Builds run on the build host; workspace rules are in the root [README](../../README.md).
