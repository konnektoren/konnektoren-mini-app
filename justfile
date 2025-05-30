# justfile

# justfile

# Import styles justfile
styles := "styles"

# Set default values for environment variables
export BUILD_DIR := env_var_or_default("BUILD_DIR", "dist")
export REPORTS_DIR := env_var_or_default("REPORTS_DIR", "reports")

# Default recipe to display help information
default:
    @just --list

# Setup everything
setup: setup-rust

# Setup Rust tools
setup-rust:
    cargo install trunk
    cargo install wasm-pack
    rustup target add wasm32-unknown-unknown

# Start development server
serve:
    trunk serve

# Build the project for release
build: styles-check
    #!/usr/bin/env bash

    # Main build
    trunk build --release

# Check styles before build
styles-check:
    cd {{styles}} && just vendor-status

setup-styles:
    cd {{styles}} && just setup-vendors

# Run all tests
test: test-cargo test-wasm

# Run cargo tests
test-cargo:
    cargo test

# Run wasm tests in Firefox
test-wasm:
    wasm-pack test --headless --firefox

# Format code
fmt:
    cargo fmt

# Check code formatting
fmt-check:
    cargo fmt --check

# Update all dependencies
update: update-rust update-styles

# Update Rust dependencies
update-rust:
    cargo update

# Update style dependencies
update-styles:
    cd {{styles}} && just update-vendors

lint:
    cargo clippy -- -D warnings
