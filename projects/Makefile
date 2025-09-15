# Makefile for building Craft projects

.PHONY: all build build-examples build-workspace clean help

# Default target
all: build

# Build everything (workspace + smart escrows)
build: build-workspace build-examples

# Build the main workspace projects
build-workspace:
	@echo "Building workspace projects..."
	cargo build
	cargo build --release
	cargo build --target wasm32-unknown-unknown
	cargo build --target wasm32-unknown-unknown --release

# Build all smart escrow examples (automatically included in workspace build via build.rs)
build-examples:
	@echo "Smart escrow examples are built automatically via build.rs"
	@echo "Just run 'cargo build' to build everything including smart escrows"

# Clean all build artifacts
clean:
	@echo "Cleaning workspace..."
	cargo clean
	@echo "Cleaning smart escrow projects..."
	@for project in examples/smart-escrows/*/; do \
		if [ -f "$$project/Cargo.toml" ]; then \
			echo "Cleaning $$project"; \
			(cd "$$project" && cargo clean); \
		fi \
	done

# Show help
help:
	@echo "Available targets:"
	@echo "  all                 - Build everything (default)"
	@echo "  build               - Build workspace and smart escrows"
	@echo "  build-workspace     - Build only workspace projects"
	@echo "  build-examples - Info about smart escrow building"
	@echo "  clean               - Clean all build artifacts"
	@echo "  help                - Show this help message"
	@echo ""
	@echo "Note: Smart escrow examples are automatically built via build.rs"
	@echo "when you run 'cargo build'. Each project maintains its own target directory."
