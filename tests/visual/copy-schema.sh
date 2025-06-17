#!/bin/bash

# Copy Schema to Clipboard for JSON Crack
# Simple, reliable approach using the web version

SCHEMA_NAME=${1:-"BaseCognitiveEntity"}
SCHEMAS_DIR="../../dist/bundled_schemas"
SCHEMA_FILE="${SCHEMAS_DIR}/${SCHEMA_NAME}.schema.json"

echo "📋 Schema Clipboard Helper"
echo "========================="

# Check if schema exists
if [ ! -f "$SCHEMA_FILE" ]; then
    echo "❌ Schema not found: $SCHEMA_FILE"
    echo ""
    echo "Available schemas:"
    ls -1 "$SCHEMAS_DIR"/*.json | sed 's/.*\///' | sed 's/\.schema\.json//' | sort
    exit 1
fi

# Copy to clipboard
pbcopy < "$SCHEMA_FILE"

echo "✅ Copied $SCHEMA_NAME schema to clipboard!"
echo "📄 File: $SCHEMA_FILE"
echo ""
echo "🎯 Next steps:"
echo "   1. Open https://jsoncrack.com/editor"
echo "   2. Paste (Cmd+V) the schema"
echo "   3. Explore the interactive visualization!"
echo ""
echo "🔍 Schema preview:"
echo "=================="
cat "$SCHEMA_FILE" | jq -r '.title, .description' | head -2 