# shenron-mcp

MCP server (stdio) exposing the dragonballfr.com `dragonball.*` and `dbfr_*` tools and the local corpus.

## Development

```sh
cargo check -p shenron-mcp
cargo nextest run -p shenron-mcp
```

Builds run on the build host; workspace rules are in the root [README](../../README.md).

## Admin data plane

The `db.*` (PostgreSQL), `redis.*` and `memory.*` tools and the real-time resources `shenron://db/table/<name>` and `shenron://events/<channel>` need the elevated level (token or allowed IP) and are absent from `tools/list` for an anonymous caller. Variables, guards, migrations and token provisioning: `docs/runbooks/shenron-mcp-admin.md` in shenron. Integration tests (`tests/admin_integration.rs`) run on `fr2` with `SHENRON_MCP_TEST_DATABASE_URL` and `SHENRON_MCP_TEST_REDIS_URL`.
