#!/bin/bash
# Generate TypeScript bindings from Rust ts-rs exports
# Run after cargo build in tauri-shared

set -e

OUTPUT_DIR="../tauri-front-shared/projects/shared/src/models/bindings"
mkdir -p "$OUTPUT_DIR"

echo "Generating TypeScript bindings..."
# ts-rs generates .ts files next to the Rust source files when #[ts(export)] is used
# We need to copy them to the frontend project

# Find all generated .ts files
find ../tauri-front-shared -name "*.ts" -path "*/bindings/*" 2>/dev/null | head -20

echo "Done. Copy binding files to $OUTPUT_DIR"
