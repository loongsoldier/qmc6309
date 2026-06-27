# QMC6309 Driver development commands

default:
    @just --list

# Re-generate register code from DSL
generate:
    device-driver-cli -m qmc6309_reg.dsl -d Qmc6309Reg -o src/register.rs

# Build all feature combinations
build:
    cargo build --target thumbv7em-none-eabihf
    cargo build --target thumbv7em-none-eabihf --features async

# Run tests
test:
    cargo test --all-features

# Clippy
lint:
    cargo clippy --target thumbv7em-none-eabihf -- -D warnings

# Pre-publish checks
check: build test lint
    @echo "All checks passed"

# Dry-run publish
publish: check
    cargo publish --dry-run
