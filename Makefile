driver:
	cargo run --bin lunar_driver

test:
	cargo run --bin test-suite

test-release:
	cargo run --bin test-suite --release

test-full:
	cargo run --bin test-suite --features scripts
