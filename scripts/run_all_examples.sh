#!/bin/bash

# Get all .rs files in examples/
EXAMPLES=$(ls examples/*.rs | sed 's/examples\///;s/\.rs//')

echo "Starting Murali Example Runner..."
echo "This will run each example one by one. Close the window to proceed to the next."
echo "----------------------------------------------------------------------------"

for EX in $EXAMPLES; do
    echo ">>> Running example: $EX"
    cargo run --example "$EX"
    
    if [ $? -ne 0 ]; then
        echo "!!! Example $EX failed with exit code $?. Press Enter to continue to next, or Ctrl+C to abort."
        read
    fi
done

echo "----------------------------------------------------------------------------"
echo "All examples finished!"
