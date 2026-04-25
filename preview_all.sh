#!/bin/bash

# This script runs all Murali examples sequentially with the --preview flag.
# Once one example window is closed, the next one will start automatically.

EXAMPLES_DIR="examples"

if [ ! -d "$EXAMPLES_DIR" ]; then
    echo "Error: examples directory not found."
    exit 1
fi

# Find all .rs files in the examples directory
for f in "$EXAMPLES_DIR"/*.rs; do
    # Extract filename without extension
    example_name=$(basename "$f" .rs)
    
    echo "===================================================="
    echo "▶ Running Example: $example_name"
    echo "===================================================="
    
    # Run the example with the preview flag
    cargo run --example "$example_name" -- --preview
    
    # Check if the process was interrupted
    status=$?
    if [ $status -ne 0 ]; then
        echo "Example $example_name exited with status $status. Stopping."
        exit $status
    fi
done

echo "Done! All examples have been previewed."
