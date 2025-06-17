#!/bin/bash

# JSON Crack Visual Testing Setup
# Production-ready script for schema visualization

set -e

JSONCRACK_DIR="../../jsoncrack.com"
SCHEMAS_DIR="../../dist/bundled_schemas"

echo "🎨 JSON Crack Visual Testing Setup"
echo "=================================="

# Check if JSON Crack directory exists
if [ ! -d "$JSONCRACK_DIR" ]; then
    echo "❌ JSON Crack not found at $JSONCRACK_DIR"
    echo "   Run: cd ../../ && git clone https://github.com/AykutSarac/jsoncrack.com.git"
    exit 1
fi

# Check if schemas exist
if [ ! -d "$SCHEMAS_DIR" ]; then
    echo "❌ Bundled schemas not found at $SCHEMAS_DIR"
    echo "   Run: cd ../../ && make bundle"
    exit 1
fi

# Check if port 3000 is available
if lsof -i :3000 > /dev/null 2>&1; then
    echo "⚠️  Port 3000 is in use. Stopping existing process..."
    lsof -ti :3000 | xargs kill -9 2>/dev/null || true
    sleep 2
fi

echo "🚀 Starting JSON Crack..."
cd "$JSONCRACK_DIR"

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    pnpm install
fi

# Start JSON Crack in background
echo "🌐 Starting server on http://localhost:3000"
pnpm dev > /dev/null 2>&1 &
JSONCRACK_PID=$!

# Wait for server to start
echo "⏳ Waiting for server to start..."
for i in {1..30}; do
    if curl -s http://localhost:3000 | grep -q "JSON Crack\|<!DOCTYPE html"; then
        echo "✅ JSON Crack is running at http://localhost:3000"
        echo "📄 Available schemas:"
        ls -1 "$SCHEMAS_DIR"/*.json | sed 's/.*\///' | sed 's/\.schema\.json//'
        echo ""
        echo "🎯 Usage:"
        echo "   1. Open http://localhost:3000 in your browser"
        echo "   2. Copy schema: pbcopy < $SCHEMAS_DIR/[schema-name].schema.json"
        echo "   3. Paste into JSON Crack editor"
        echo ""
        echo "🛑 To stop: kill $JSONCRACK_PID"
        exit 0
    fi
    sleep 1
done

echo "❌ Failed to start JSON Crack"
kill $JSONCRACK_PID 2>/dev/null || true
exit 1 