# Build Bounce Back Android APK

Install Rust, the Android SDK/NDK, and `cargo-quad-apk` to build locally, or use the GitHub Actions workflow.

## Local command

```bash
rustup target add aarch64-linux-android
cargo install cargo-quad-apk --locked
cargo quad-apk build --release
```

## CI

`.github/workflows/android-apk.yml` installs:
- Android platform 35
- Android build-tools 35.0.0
- Android NDK 27.2.12479018
- Rust stable with `aarch64-linux-android`
- `cargo-quad-apk`

The release APK is uploaded as a GitHub Actions artifact named `Bounce-Back-release-apk`.
