#!/bin/bash

# This script exports all Murali examples sequentially with the --export flag.
# Each example writes its artifacts under rendered_output/ before the next one starts.

set -u

EXAMPLES_DIR="examples"

if [ ! -d "$EXAMPLES_DIR" ]; then
    echo "Error: examples directory not found."
    exit 1
fi

run_mode="debug"
if [ "${1:-}" = "--release" ]; then
    run_mode="release"
    shift
fi

extra_args=("$@")

for f in "$EXAMPLES_DIR"/*.rs; do
    example_name=$(basename "$f" .rs)

    echo "===================================================="
    echo "▶ Exporting Example: $example_name ($run_mode)"
    echo "===================================================="

    cargo_cmd=(cargo run)
    if [ "$run_mode" = "release" ]; then
        cargo_cmd+=(--release)
    fi
    cargo_cmd+=(--example "$example_name" -- --export)

    if [ "${#extra_args[@]}" -gt 0 ]; then
        cargo_cmd+=("${extra_args[@]}")
    fi

    "${cargo_cmd[@]}"

    status=$?
    if [ $status -ne 0 ]; then
        echo "Example $example_name exited with status $status. Stopping."
        exit $status
    fi
done

echo "Done! All examples have been exported."
