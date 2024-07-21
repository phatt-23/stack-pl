#!/bin/env bash

# path to my compiler/interpreter
compiler=./target/debug/stack_based_PL

# output files
sim_output=test_for_diffs_output.sim.txt
com_output=test_for_diffs_output.com.txt

rm $sim_output $com_output

for file in lang_src/*.p; do
    if [[ -e "$file" ]]; then
        echo "$file"
        $compiler --sim       $file >> $sim_output
        $compiler --com --run $file >> $com_output
    fi
done
