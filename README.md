# Bounce Back — Rust reconstruction

A playable cross-platform reconstruction of the supplied *Bounce Back* v1.13 J2ME game. The project includes the recovered 22-level map set and extracted art in `assets/`.

## Play

- **Move:** Left/Right arrows, A/D, or 4/6
- **Jump:** Up, W, Space, or 2/5
- **Touch:** the lower left and center zones move; the lower right zone jumps
- **Pause:** Escape or the pause control in the upper-right corner

Choose **Start Game** to begin at level one, or **Level Select** to choose a level. Collect all point items to clear a level and move on. You have three lives; falling out of the map or touching a hazard costs one life and restarts the current level.

The Records screen tracks the best score for the current app session.

## Build

Desktop:

```text
cargo run
```

Android requires Rust, the Android SDK/NDK, and `cargo-quad-apk`:

```text
rustup target add aarch64-linux-android
cargo install cargo-quad-apk --locked
cargo quad-apk build --release
```

The GitHub Actions web workflow builds the WebAssembly game and publishes it to GitHub Pages on pushes to `main` (or when started manually). The Android workflow remains available separately.

## Reconstruction notes

The runtime uses a 50 ms fixed-step update, 16 px world cells, extracted player animation frames, recovered level metadata and tile maps, and extracted theme art. Bonus items use the recovered score values. Unknown tile behaviors and moving-object records are still approximated; the original Nokia renderer and every game mechanic are not yet reproduced pixel-for-pixel.
