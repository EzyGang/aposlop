.PHONY: format check aposlop

format:
	@cargo fmt --all
	@cargo clippy --all-targets --workspace --fix --allow-dirty

check:
	@cargo check --workspace
	@cargo fmt --all -- --check
	@cargo clippy --all-targets --workspace -- -D warnings
	@cargo test --workspace
	@cargo run -- ci .

aposlop:
	@cargo run -- .