set dotenv-load

# List wiki recipes
default:
    @just --list

# Fast workspace check
check:
    cargo check --workspace

# Clippy with warnings denied (Zed / Fw standard)
lint:
    cargo clippy --workspace --all-targets -- -D warnings

# Format Rust sources
fmt:
    cargo fmt --all

# Check formatting without modifying files
fmt-check:
    cargo fmt --all -- --check

# Run full workspace test suite
test:
    cargo test --workspace

# Release build (cosmic-epoch pattern)
build-release:
    cargo build --release --workspace
