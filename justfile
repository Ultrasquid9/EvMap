default: lint run

lint:
	cargo fmt
	cargo clippy

run:
	cargo run
