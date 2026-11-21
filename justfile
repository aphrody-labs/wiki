set dotenv-load

# List wiki recipes
default:
    @just --list

# Type-check every crate and target
check:
    cargo ck

# Clippy with warnings denied
lint:
    cargo lint

# Format Rust sources
fmt:
    cargo fmt --all

# Check formatting without modifying files
fmt-check:
    cargo fmt --all -- --check

# Run the workspace tests
test:
    cargo t

# Release build
build-release:
    cargo build --release --workspace
