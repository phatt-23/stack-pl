#!/bin/env bash

# Path to my compiler/interpreter
compiler=./target/debug/stack_based_PL

# Dir of my examples
src_dir=examples

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
for file in $src_dir/*.p; do
    if [[ -e "$file" ]]; then
        echo "[TEST] Cmp: $file"
        # Compile and run simulation
        $compiler -c $file -o $program
        $program > $com_output
        $compiler -s $file > $sim_output

        # Compare outputs
        diff_output=$(diff $sim_output $com_output)
        if [[ -z "$diff_output" ]]; then 
            echo "[TEST] Okay: simulation == compilation"
        else
            echo "[TEST] Diff: simulation != compilation"
            if [ "$show" = true ]; then
                echo "[SHOW]:"
                diff --color=always $sim_output $com_output
            fi
        fi
    
    fi
done
