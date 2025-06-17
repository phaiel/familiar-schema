#!/bin/bash

# Professional JSON Crack integration script
# Usage: ./view-in-jsoncrack.sh [schema_name]
# Example: ./view-in-jsoncrack.sh BaseCognitiveEntity

SCHEMA_NAME=${1:-"BaseCognitiveEntity"}
VISUAL_DIR="../../dist/visual"
SCHEMA_FILE="${VISUAL_DIR}/${SCHEMA_NAME}.formatted.json"

if [ ! -f "$SCHEMA_FILE" ]; then
    echo "❌ Schema file not found: $SCHEMA_FILE"
    echo "📋 Available schemas:"
    ls -1 "$VISUAL_DIR"/*.formatted.json 2>/dev/null | xargs -n1 basename | sed 's/.formatted.json//' || echo "   No schemas found"
    exit 1
fi

echo "📋 Copying ${SCHEMA_NAME} schema to clipboard..."
pbcopy < "$SCHEMA_FILE"

echo "✅ Schema copied to clipboard!"
echo "🌐 Opening JSON Crack..."
open "https://jsoncrack.com/editor"

echo ""
echo "📝 Instructions:"
echo "   1. Paste (Cmd+V) the schema into JSON Crack"
echo "   2. Click 'Visualize' to see the schema diagram"
echo "   3. Use the controls to explore your schema structure" 