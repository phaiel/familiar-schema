# ==============================================================================
# Canonical Makefile for the Familiar v3 Schema-to-Code Generation Pipeline
# ==============================================================================

# --- Configuration ---
V3_DIR := docs/v3
SCHEMA_DIR := $(V3_DIR)/schemas
RUNNER_DIR := $(V3_DIR)/scripts/runner
RUST_GENERATED_FILE := src/generated/familiar_types.rs
PYTHON_REQUIREMENTS := $(V3_DIR)/scripts/requirements.txt

# --- Primary Targets ---
.PHONY: all setup assemble validate generate-types generate-catalog generate-docs ci clean

# The full pipeline: assemble, validate, then generate code and catalog.
all: assemble validate generate-types generate-catalog generate-docs

# The CI pipeline runs the same full flow.
ci: setup all

# Install Python dependencies and build the Rust codegen tool.
setup:
	@echo "🐍 Installing Python dependencies..."
	@pip install --upgrade pip
	@pip install -r $(PYTHON_REQUIREMENTS)
	@echo "✅ Python dependencies installed."
	@echo "🦀 Building Rust code generation tool (schemafy)..."
	@cd $(RUNNER_DIR) && cargo build --release
	@echo "✅ schemafy runner built."

# Step 1: Assemble and dereference schemas using the robust Python script.
assemble:
	@echo "🧩 Assembling and dereferencing schemas..."
	@python3 $(V3_DIR)/scripts/assemble_schemas.py \
		--source-dir $(SCHEMA_DIR) \
		--output-dir $(SCHEMA_DIR)/assembled
	@echo "✅ Schema assembly complete."

# Step 2: Validate the assembled schemas to ensure they are well-formed.
validate: assemble
	@echo "🔍 Validating assembled schemas with check-jsonschema..."
	@check-jsonschema --check-metaschema $$(find $(SCHEMA_DIR)/assembled -name "*.json")
	@echo "✅ Schemas validated."

# Step 3: Generate pure Rust types from the source schemas using schemafy.
generate-types: setup assemble
	@echo "🏗️ Generating Rust types with schemafy..."
	@mkdir -p $(dir $(RUST_GENERATED_FILE))
	@cd $(RUNNER_DIR) && cargo run --release -- \
		-o ../../../../$(RUST_GENERATED_FILE) \
		../../schemas/assembled/FamiliarTypes.schema.json
	@echo "✅ Rust types generated in $(RUST_GENERATED_FILE)."

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
	@rm -rf $(SCHEMA_DIR)/assembled
	@rm -rf $(V3_DIR)/dist
	@rm -f $(RUST_GENERATED_FILE)
	@rm -f catalog-info.yaml
	@rm -f mkdocs.yml
	@rm -rf docs/index.md
	@rm -rf $(RUNNER_DIR)/target
	@rm -f $(RUNNER_DIR)/Cargo.lock
	@echo "✅ Cleanup complete."

# Docker Compose Target for Schema Registry (if needed)
# start-registry:
# ... existing code ... 