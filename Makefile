run:
	cargo run --manifest-path day-$(day)/Cargo.toml --bin part-$(part)

generate:
	cargo run --manifest-path scripts/Cargo.toml --bin generate
