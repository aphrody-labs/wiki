# wiki

Rust-only workspace extracted from shenron (shenron-*, koharu-*, dragon-pixel, xtask). Shenron itself
is full Bun and owns `data/`, `packages/`, `apps/`; nothing Bun lives here.

## Layout

- `crates/*`: workspace members (excluded: shenron-video, shenron-tv, shenron-deployd, standalone).
- `.cargo/config.toml`: aliases and `SHENRON_DATA_DIR` (../shenron/data). `.cargo/local.toml` is
  untracked and machine local.
- `replica/`: product recon for dragonballfr.com, read by no crate.

## Gates

`cargo metadata --no-deps --offline`, `cargo ck`, `cargo lint`, `cargo t`. Heavy builds on `fr1`
(sibling `shenron` checkout with `data/` required next to `wiki`; `nie` for `shenron-re`).

## Data contract with shenron

- Read shenron data only through `SHENRON_DATA_DIR`; never copy `data/` here and never hard-code a
  `../../data` path.
- Other shenron files (openapi, templates, generated TypeScript): `SHENRON_ROOT`, default the parent
  of `SHENRON_DATA_DIR`.
- Generated TypeScript of `shenron-ffi` is written into shenron (`UPDATE=1`, `packages/native`).

## Git

Work on `main`, commit by explicit path, no branches, stash or reset, no AI attribution trailers.
