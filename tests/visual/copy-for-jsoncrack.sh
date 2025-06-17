#!/bin/bash

# Copy Formatted Schema for JSON Crack
# Production script that uses the build pipeline artifacts

SCHEMA_NAME=${1:-"BaseCognitiveEntity"}
VISUAL_DIR="../../dist/visual"
SCHEMA_FILE="${VISUAL_DIR}/${SCHEMA_NAME}.formatted.json"

echo "📋 JSON Crack Schema Helper"
echo "=========================="

# Check if visual artifacts exist
if [ ! -d "$VISUAL_DIR" ]; then
    echo "❌ Visual artifacts not found. Run: make visualize"
    exit 1
fi

# Check if specific schema exists
if [ ! -f "$SCHEMA_FILE" ]; then
    echo "❌ Schema not found: $SCHEMA_FILE"
    echo ""
    echo "Available schemas:"
    ls -1 "$VISUAL_DIR"/*.formatted.json 2>/dev/null | sed 's/.*\///' | sed 's/\.formatted\.json//' | sort
    exit 1
fi

# Copy to clipboard
pbcopy < "$SCHEMA_FILE"

echo "✅ Copied $SCHEMA_NAME formatted schema to clipboard!"
echo "📄 File: $SCHEMA_FILE"
echo ""
echo "🎯 Next steps:"
echo "   1. Open http://localhost:3000 (JSON Crack)"
echo "   2. Paste (Cmd+V) the schema"
echo "   3. Explore the visualization!"
echo ""
echo "🔄 To regenerate: make visualize" 