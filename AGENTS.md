# wiki

Rust-only workspace extracted from shenron (shenron-*, koharu-*, dragon-pixel, xtask). Shenron itself
is full Bun and owns `data/`, `packages/`, `apps/`; nothing Bun lives here. Layout, data contract and
generated TypeScript flow: [README.md](README.md).

## Rules

- Gates: `cargo metadata --no-deps --offline`, `cargo ck`, `cargo lint`, `cargo t` (aliases in `.cargo/config.toml`). Heavy builds on `fr1`
  (sibling `shenron` checkout with `data/` required next to `wiki`; `nie` for `shenron-re`).
- Read shenron data only through `SHENRON_DATA_DIR`; never copy `data/` here and never hard-code a `../../data` path.
  `.cargo/local.toml` is untracked and machine local.
- Excluded from the workspace (own `Cargo.lock`): shenron-video, shenron-tv, shenron-deployd.
- `replica/`: product recon for dragonballfr.com, read by no crate.
- Git: work on `main`, commit by explicit path, no branches, stash or reset. Message format and attribution: the org standard,
  [aphrody-labs/.github](https://github.com/aphrody-labs/.github) (`docs/standards/commit-format.md`).
