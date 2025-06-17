#!/bin/bash

# Pipeline Test Script for Familiar v3 Schema-First Workflow
# Tests the complete pipeline from schema validation to code generation

set -e  # Exit on any error

echo "🧪 Testing Familiar v3 Schema Pipeline..."
echo "========================================"

# Test 1: Schema Validation
echo "📋 Test 1: Schema Validation"
make validate
echo "✅ Schema validation passed"
echo

# Test 2: Schema Bundling  
echo "📋 Test 2: Schema Bundling"
make bundle

# Verify bundled files exist
if [ ! -f "dist/bundled_schemas/BaseCognitiveEntity.schema.json" ]; then
    echo "❌ BaseCognitiveEntity bundled schema not found"
    exit 1
fi

if [ ! -f "dist/bundled_schemas/BaseSystemEntity.schema.json" ]; then
    echo "❌ BaseSystemEntity bundled schema not found"
    exit 1
fi

echo "✅ Schema bundling passed"
echo

# Test 3: Type Generation
echo "📋 Test 3: Type Generation"
make generate-types

# Verify generated Rust file exists and has content
if [ ! -f "../../src/generated/types/familiar_types.rs" ]; then
    echo "❌ Generated Rust types file not found"
    exit 1
fi

# Check if the file has meaningful content (more than just comments)
rust_lines=$(grep -v "^//" ../../src/generated/types/familiar_types.rs | grep -v "^$" | wc -l)
if [ "$rust_lines" -lt 10 ]; then
    echo "❌ Generated Rust file appears to be empty or too small"
    exit 1
fi

echo "✅ Type generation passed"
echo

# Test 4: Full Pipeline
echo "📋 Test 4: Full Pipeline Integration"
make clean
make all
echo "✅ Full pipeline integration passed"
echo

# Test 5: Verify Generated Code Quality
echo "📋 Test 5: Generated Code Quality Check"

# Check for essential Rust elements
if ! grep -q "use serde" ../../src/generated/types/familiar_types.rs; then
    echo "❌ Generated code missing serde imports"
    exit 1
fi

if ! grep -q "#\[derive(" ../../src/generated/types/familiar_types.rs; then
    echo "❌ Generated code missing derive macros"
    exit 1
fi

if ! grep -q "pub struct" ../../src/generated/types/familiar_types.rs; then
    echo "❌ Generated code missing public structs"
    exit 1
fi

echo "✅ Generated code quality check passed"
echo

# Summary
echo "🎉 ALL TESTS PASSED!"
echo "==================="
echo "✅ Schema validation works"
echo "✅ Schema bundling works" 
echo "✅ Type generation works"
echo "✅ Full pipeline integration works"
echo "✅ Generated code quality is good"
echo
echo "📊 Pipeline Statistics:"
echo "- Schemas validated: $(find schemas -name "*.schema.json" | wc -l | tr -d ' ')"
echo "- Schemas bundled: $(ls dist/bundled_schemas/*.json 2>/dev/null | wc -l | tr -d ' ')"
echo "- Generated Rust lines: $(wc -l < ../../src/generated/types/familiar_types.rs | tr -d ' ')"
echo
echo "🚀 The Familiar v3 Schema Pipeline is working correctly!" 