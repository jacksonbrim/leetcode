#!/bin/bash

# Name of the output file
MOD_FILE="mod.rs"

# Clear the mod.rs file if it exists or create a new one
> "$MOD_FILE"

# Iterate over all .rs files in the current directory
for file in *.rs; do
    # Skip the mod.rs file itself
    if [ "$file" == "$MOD_FILE" ]; then
        continue
    fi

    # Extract the base name without the .rs extension
    module_name=$(basename "$file" .rs)

    # Append the formatted string to mod.rs
    echo "pub mod $module_name;" >> "$MOD_FILE"
done

echo "mod.rs file has been updated."

