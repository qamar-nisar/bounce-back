run:
	cargo run

release:
	cargo build --release

android:
	cargo quad-apk build --release
