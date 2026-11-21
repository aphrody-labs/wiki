# Parity fixtures: provenance

`tests/ts_parity_golden.rs` replays these files through the Rust ports (`shenron_rag_core::eval`,
`shenron_rag_core::MarkdownChunker` and the UTF-16 length unit) and asserts the outputs match exactly
(floats with a tight epsilon). `sans-tag` in the TypeScript output is renamed `untagged` on the Rust side
(English-only identifiers); the test remaps the key before comparing.

- `eval_golden.json`: generated on 2026-09-19 by a throwaway Bun script that imported shenron's
  `apps/bot/src/lib/rag-eval.ts` and called `runCase`, `aggregate`, `aggregateByTag`, `normalizeUrl` and
  `matchesExpectedUrl` on a small hand-built set of queries and hits.
- `prose_chunk_golden.json`: generated the same day by a throwaway Bun script that ran `chunkDocument` of
  shenron's `_rag-chunk.ts` (no longer in `apps/bot/scripts`) over a small Markdown document with
  `maxChunkChars = 220`, `overlapPct = 0.15`.
- `unicode_length_golden.json`: `chunkDocument` over a French accented paragraph (more than 3000 UTF-16 units) with
  `maxChunkChars = 1400`, `overlapPct = 0.15`, run with Bun on 2026-09-19; its script is not recorded.

To regenerate, recreate the scripts from the descriptions above against the shenron commit that still has
those sources, and replace the files.
