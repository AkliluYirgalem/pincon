NIGHTLY_TOOLCHAIN := "nightly-2026-01-23"
SOLANA_VERSION := "3.0.13"

# Build Solana test programs
build-test-programs:
	@cargo build-sbf --manifest-path tests/signer/Cargo.toml
	@cargo build-sbf --manifest-path tests/writable/Cargo.toml

# Run clippy checks
clippy:
	@cargo +{{NIGHTLY_TOOLCHAIN}} clippy --all-targets --all-features -- -D warnings

# Auto-fix clippy issues
clippy-fix:
	@cargo +{{NIGHTLY_TOOLCHAIN}} clippy --all --all-features --all-targets --fix --allow-dirty --allow-staged -- -D warnings
	
# Check formatting
format:
	@cargo +{{NIGHTLY_TOOLCHAIN}} fmt --all -- --check

# Fix formatting
format-fix:
	@cargo +{{NIGHTLY_TOOLCHAIN}} fmt --all

# Test all feature combos
hack:
	@cargo hack check --feature-powerset --all-targets

# Echo nightly version
nightly-version:
	@echo {{NIGHTLY_TOOLCHAIN}}

# Echo Solana version
solana-version:
	@echo {{SOLANA_VERSION}}

# Build then run tests
test:
	@just build-test-programs
	@cargo test --manifest-path tests/signer/Cargo.toml --all-features
	@cargo test --manifest-path tests/writable/Cargo.toml --all-features
