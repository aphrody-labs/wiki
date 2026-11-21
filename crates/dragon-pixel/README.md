# dragon-pixel

Shenron fighting game: deterministic combat engine, arcade simulation and WebGPU render packets.

## Development

```sh
cargo check -p dragon-pixel
cargo nextest run -p dragon-pixel
```

Builds run on the build host; workspace rules are in the root [README](../../README.md).

## Bevy game in the browser

The `bevy` feature (wasm32 and WebGPU) compiles `bevy_view.rs`: Bevy reads keyboard and gamepads, advances `DragonPixelEngineCore` at a fixed 60 Hz and draws the state in 3D. JavaScript only starts, pauses and reads the HUD (`dragon_pixel_*`). The same module exposes `DragonPixelEngineWasm` and `map_standard_gamepad`, which the site uses for its 2D canvas fallback without WebGPU.

The site build is cargo for `wasm32-unknown-unknown` with the `bevy` feature, `wasm-bindgen --target web --no-typescript --out-name dragon_pixel` and `wasm-opt -Oz`, run on `fr1` or in CI into `target/wasm/dragon-pixel`. Shenron stages the output with content-hashed names (`just dragon-pixel-stage`) and checks it with `just dragon-pixel-wasm-check`.
