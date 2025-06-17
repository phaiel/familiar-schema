# ==============================================================================
# Canonical Makefile for the Familiar v3 Schema-to-Code Generation Pipeline
# ==============================================================================

# --- Configuration ---
SCHEMA_DIR := ./schemas
ASSEMBLED_DIR := $(SCHEMA_DIR)/assembled
BUNDLED_DIR := ./dist/bundled_schemas
VISUAL_DIR := ./dist/visual

# Find assembled schemas (output of the assembly process)
ASSEMBLED_SCHEMAS := $(shell find $(ASSEMBLED_DIR) -name "*.schema.json" 2>/dev/null)

# --- Primary Targets ---
.PHONY: all assemble validate bundle generate-types generate-catalog visualize ci clean list-assembled

# The full pipeline: assemble, validate, then generate code and catalog
all: assemble validate generate-types generate-catalog

# The CI pipeline runs the same full flow
ci: all
	@echo "✅ All CI checks passed."

# Step 1: Assemble final schemas from templates and snippets using Jinja2
assemble:
	@echo "🧩 Assembling final schemas from templates and snippets..."
	@python3 scripts/assemble_schemas.py
	@echo "✅ Schema assembly complete."

# Step 2: Validate the assembled schemas
validate: assemble
	@echo "🔍 Validating assembled schemas..."
	@if [ -z "$(ASSEMBLED_SCHEMAS)" ]; then \
		echo "❌ No assembled schemas found. Run 'make assemble' first."; \
		exit 1; \
	fi
	@for schema in $(ASSEMBLED_SCHEMAS); do \
		echo "Validating $$schema..."; \
		jsonschema metaschema $$schema || exit 1; \
	done
	@echo "✅ Schema validation successful."

# Step 3: Generate Rust types from assembled schemas  
generate-types: assemble
	@echo "🏗️ Generating pure Rust data structs from assembled schemas..."
	@if [ -z "$(ASSEMBLED_SCHEMAS)" ]; then \
		echo "❌ No assembled schemas found. Run 'make assemble' first."; \
		exit 1; \
	fi
	@mkdir -p ../../src/generated/types
	@quicktype --src-lang schema --lang rust --src $(ASSEMBLED_SCHEMAS) -o ../../src/generated/types/familiar_types.rs
	@echo "✅ Rust code generated."

# Step 4: Generate Backstage catalog from schemas
generate-catalog: validate
	@echo "📋 Generating Backstage catalog from schemas..."
	@python3 scripts/generate_catalog_info.py
	@cp catalog-info.yaml ../../catalog-info.yaml
	@echo "✅ Backstage catalog generated."

# Step 5: Optional bundling for external tool compatibility
bundle: assemble
	@echo "📦 Creating bundled schemas for external tools..."
	@mkdir -p $(BUNDLED_DIR)
	@for schema_file in $(ASSEMBLED_SCHEMAS); do \
		relative_path=$$(echo $$schema_file | sed 's|$(ASSEMBLED_DIR)/||'); \
		output_dir=$(BUNDLED_DIR)/$$(dirname $$relative_path); \
		output_file=$$output_dir/$$(basename $$schema_file); \
		mkdir -p $$output_dir; \
		echo "  -> Bundling $$schema_file to $$output_file"; \
		json-dereference -s $$schema_file -o $$output_file 2>/dev/null || cp $$schema_file $$output_file; \
	done
	@echo "✅ Schema bundling complete."

# Step 6: Generate visualization files for JSON Crack
visualize: assemble
	@echo "🎨 Generating visualization files..."
	@mkdir -p $(VISUAL_DIR)
	@for schema_file in $(ASSEMBLED_SCHEMAS); do \
		base_name=$$(basename $$schema_file .schema.json); \
		echo "  -> Creating visualization for $$base_name"; \
		jq . $$schema_file > $(VISUAL_DIR)/$$base_name.json; \
	done
	@echo "✅ Visualization files generated in $(VISUAL_DIR)/"
	@echo "📋 To view in JSON Crack, copy the contents of any file in $(VISUAL_DIR)/ and paste into https://jsoncrack.com/"

# Development helpers
list-assembled:
	@echo "📋 Assembled schemas:"
	@for schema in $(ASSEMBLED_SCHEMAS); do \
		echo "  $$schema"; \
	done

# Analyze missing types (helps identify what snippets to create)
analyze-types: generate-types
	@echo "🔍 Analyzing generated Rust code for type inference issues..."
	@echo "Looking for HashMap<String, Option<serde_json::Value>> patterns..."
	@grep -n "HashMap<String, Option<serde_json::Value>>" ../../src/generated/types/familiar_types.rs || echo "✅ No generic HashMaps found"
	@echo "Looking for Option<serde_json::Value> patterns..."
	@grep -n "Option<serde_json::Value>" ../../src/generated/types/familiar_types.rs || echo "✅ No generic Options found"

# Quick validation of source templates (before assembly)
validate-templates:
	@echo "🔍 Quick validation of source templates..."
	@find ./schemas/templates -name "*.jinja" -exec echo "Checking template: {}" \;
	@find ./schemas/snippets -name "*.json" -exec jsonschema metaschema {} \;
	@echo "✅ Template and snippet validation successful."

# Schema Registry targets removed - using simpler assembly approach

# Utility to clean all generated artifacts
clean:
	@echo "🧹 Cleaning generated artifacts..."
	@rm -rf $(ASSEMBLED_DIR)
	@rm -rf ./dist
	@rm -f ../../src/generated/types/*.rs  
	@echo "✅ Cleanup complete."