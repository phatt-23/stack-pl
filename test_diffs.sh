#!/bin/env bash

# Path to my compiler/interpreter
compiler=./target/debug/stack_based_PL

# Directory
dir=test_diff

# Output files
sim_output=$dir/test_for_diffs_output.sim.txt
com_output=$dir/test_for_diffs_output.com.txt
program=$dir/program

mkdir -p $dir
rm -f $sim_output $com_output

show=false

# Handle flags
while [ $# -gt 0 ]; do
    case $1 in
        -h | --help)
            echo "[HELP] Usage: $0 [-s | --show]"
            exit 0
        ;;
        -s | --show)
            show=true
        ;;
    esac
    shift
done

# Process each .p file in lang_src directory
for file in lang_src/*.p; do
    if [[ -e "$file" ]]; then
        echo "[TEST info] comparing: $file"
        # Compile and run simulation
        $compiler -c $file -o $program
        $compiler -s $file > $sim_output
        $program > $com_output
    fi
done

# Compare outputs
diff_output=$(diff $sim_output $com_output)

if [[ -z "$diff_output" ]]; then 
    echo "[TEST info] okay: simulation == compilation"
else
    echo "[TEST info] discrepancy: simulation != compilation"
    if [ "$show" = true ]; then
        echo "[SHOW]:"
        diff --color=always $sim_output $com_output
    fi
fi
