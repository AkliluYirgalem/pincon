NIGHTLY_TOOLCHAIN := nightly-2026-01-23

# --- Help Menu ---
.PHONY: help
help:
	@echo "Available commands:"
	@echo "  make format           - Run rustfmt"
	@echo "  make clippy           - Run clippy"
	@echo "  make build-test-programs - Build SBF programs for testing"
	@echo "  make test             - Run cargo tests"
	@echo "  make check-features    - Check all feature combinations"
	@echo "  make nightly-version  - Print the required nightly toolchain"
	@echo "  make solana-version   - Print the required Solana version"

# --- Versioning (Used by CI) ---
.PHONY: nightly-version
nightly-version:
	@echo $(NIGHTLY_VERSION)

.PHONY: solana-version
solana-version:
	@echo $(SOLANA_VERSION)

# --- Linting & Formatting ---
.PHONY: format
format:
	cargo +$(NIGHTLY_TOOLCHAIN) fmt --all -- --check

format-fix:
	cargo +$(NIGHTLY_TOOLCHAIN) fmt --all

.PHONY: clippy
clippy:
	cargo +$(NIGHTLY_TOOLCHAIN) clippy --all-targets --all-features -- -D warnings

clippy-fix:
	cargo +$(NIGHTLY_TOOLCHAIN) clippy --all --all-features --all-targets --fix --allow-dirty --allow-staged -- -D warnings

# --- Feature Checking ---
.PHONY: check-features
check-features:
	@cargo hack check --feature-powerset --all-targets

# --- Building & Testing ---
.PHONY: build-test-programs
build-test-programs:
	cargo build-sbf --manifest-path tests/*/Cargo.toml

.PHONY: test
test:
	@$(MAKE) build-test-programs
	@cargo test --manifest-path tests/*/Cargo.toml --all-features
