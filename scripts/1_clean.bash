#!/bin/bash

# Loop through all subdirectories in the current directory
find . -type d | while read -r dir; do
    # Check if Cargo.toml exists in the subdirectory
    if [[ -f "$dir/Cargo.toml" ]]; then
        echo "Found Cargo.toml in $dir. Running cargo clean..."
        (
            cd "$dir" || exit 1
            cargo clean
        )
    fi
done

echo "Done cleaning Rust projects."
