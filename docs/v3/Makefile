# ==============================================================================
# Canonical Makefile for the Familiar v3 Schema-to-Code Generation Pipeline
# ==============================================================================

# --- Configuration ---
V3_DIR := docs/v3
SCHEMA_DIR := $(V3_DIR)/schemas
ASSEMBLED_DIR := $(SCHEMA_DIR)/assembled
RUST_GENERATED_DIR := src/generated
PYTHON_REQUIREMENTS := $(V3_DIR)/scripts/requirements.txt

# --- Primary Targets ---
.PHONY: all setup assemble validate generate-types generate-catalog generate-docs ci clean

# The full pipeline: assemble, validate, then generate code and catalog.
all: assemble validate generate-types

# The CI pipeline runs the same full flow.
ci: setup all

# Install Python dependencies and build the Rust codegen tool.
setup:
	@echo "🐍 Installing Python dependencies..."
	@pip install --upgrade pip --quiet
	@pip install -r $(PYTHON_REQUIREMENTS) --quiet
	@echo "✅ Python dependencies installed."
	@echo "🦀 Installing Rust code generation tool (typify)..."
	@cargo install cargo-typify --quiet
	@echo "✅ typify installed."

# Step 1: Assemble and dereference ALL schemas (including snippets) using the comprehensive script.
assemble:
	@echo "🧩 Assembling and dereferencing ALL schemas (including snippets)..."
	@python3 scripts/assemble_all_schemas.py \
		--source-dir schemas \
		--output-dir schemas/assembled
	@echo "✅ Comprehensive schema assembly complete."

# Step 2: Validate the assembled schemas to ensure they are well-formed.
validate: assemble
	@echo "🔍 Validating assembled schemas..."
	@for schema in schemas/assembled/*.schema.json; do \
		echo "  Validating $$schema..."; \
		check-jsonschema --check-metaschema "$$schema"; \
	done
	@echo "✅ Schemas validated."

# Step 3: Generate individual Rust files from each schema.
generate-types: setup assemble
	@echo "🏗️ Generating individual Rust types with typify..."
	@mkdir -p src/generated
	@rm -f src/generated/*.rs
	@for schema in schemas/assembled/*.schema.json; do \
		schema_name=$$(basename "$$schema" .schema.json); \
		schema_lower=$$(echo "$$schema_name" | tr '[:upper:]' '[:lower:]'); \
		echo "  Generating $$schema_lower.rs..."; \
		cargo typify \
			-o src/generated/$$schema_lower.rs \
			"$$schema"; \
	done
	@echo "  Generating mod.rs..."
	@echo "// Auto-generated module declarations" > src/generated/mod.rs
	@for schema in schemas/assembled/*.schema.json; do \
		schema_name=$$(basename "$$schema" .schema.json); \
		schema_lower=$$(echo "$$schema_name" | tr '[:upper:]' '[:lower:]'); \
		echo "pub mod $$schema_lower;" >> src/generated/mod.rs; \
		echo "pub use $$schema_lower::$$schema_name;" >> src/generated/mod.rs; \
	done
	@echo "✅ Individual Rust types generated in src/generated/."

# Step 4: Generate Backstage catalog from schemas.
generate-catalog:
	@echo "📋 Generating Backstage catalog..."
	@python3 $(V3_DIR)/scripts/generate_catalog_info.py \
		--schemas-dir $(SCHEMA_DIR) \
		--output catalog-info.yaml
	@echo "✅ Backstage catalog generated."

# Step 5: Generate comprehensive documentation.
generate-docs:
	@echo "📚 Generating documentation from schemas..."
	@python3 $(V3_DIR)/scripts/generate_docs.py \
		--schemas-dir $(abspath $(SCHEMA_DIR)) \
		--docs-dir $(abspath $(V3_DIR)/../..)
	@echo "✅ Documentation generated."

# Utility to clean all generated artifacts.
clean:
	@echo "🧹 Cleaning generated artifacts..."
	@rm -rf $(ASSEMBLED_DIR)
	@rm -rf $(RUST_GENERATED_DIR)
	@echo "✅ Cleanup complete."
	@rm -f catalog-info.yaml
	@rm -f mkdocs.yml
	@rm -rf docs/index.md
	@echo "✅ Cleanup complete."

# Docker Compose Target for Schema Registry (if needed)
# start-registry:
# ... existing code ... 