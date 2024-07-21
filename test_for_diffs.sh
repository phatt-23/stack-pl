#!/bin/env bash

# path to my compiler/interpreter
compiler=./target/debug/stack_based_PL

for file in lang_src/*.p; do
    if [[ -e "$file" ]]; then
        echo "$file"
        $compiler sim $file >> output.txt
    fi
done
