#!/bin/bash

for subdir in mhuszar-0*/; do
    for exdir in "$subdir"ex*/; do
        if [[ -f "$exdir/Cargo.toml" ]]; then
            git add "$exdir/Cargo.toml"
        fi

        if [[ -f "$exdir/src/lib.rs" ]]; then
            git add "$exdir/src/lib.rs"
        fi

        if [[ -f "$exdir/src/main.rs" ]]; then
            git add "$exdir/src/main.rs"
        fi
    done
done