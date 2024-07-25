#!/bin/env bash

# Used for testing the outputs of compiled programs from Toi language

diff_show=false
force_change=false
clean=false
silent=false

# Handle user input flags
while [ $# -gt 0 ]; do
    case $1 in
        -h | --help)
            echo "[HELP] Usage: $0 [-d | --diff]    Show the differences"
            echo "[HELP]        $0 [-f | --force]   Force change (new -> old), required for the first run"
            echo "[HELP]        $0 [-c | --clean]   Remove the test directory"
            echo "[HELP]        $0 [-s | --silent]  Only echo if outputs are different"
            exit 0
        ;;
        -d | --diff)
            diff_show=true
        ;;
        -f | --force)
            force_change=true
        ;;
        -c | --clean)
            clean=true
        ;;
        -s | --silent)
            silent=true
        ;;
    esac
    shift
done

### Variables #################################

# Path to my compiler
compiler=./target/debug/stack_based_PL

# Directory of my examples
src_dir=examples

# Path of Toi stdlib
stdlib=./stdlib/std

# Difference control main directory
test_dir=test

# Difference control subdirectories
new_output_dir=out.new
old_output_dir=out.old

# Difference control file suffixes
new_output=out.new.txt
old_output=out.old.txt

# Path of output executable
program=$test_dir/program

### /Variables #################################

# Remove the test directory
if [ "$clean" == true ]; then
    rm -r $test_dir
    exit 0
fi

# Create if not created already
mkdir -p $test_dir
mkdir -p $test_dir/$old_output_dir

# Remove the new output, clear the file
if [[ -d "$test_dir/$new_output_dir" ]]; then
    rm -r $test_dir/$new_output_dir
fi

# Always create new output in which we write the current output
mkdir -p $test_dir/$new_output_dir

# Create old_output if it doesnt exist
if [[ ! -f "$test_dir/$old_output_dir" ]]; then
    touch $test_dir/$old_output_dir
fi

# Process each source file in src_dir directory
for file in $src_dir/*.p; do
    if [[ -f "$file" ]]; then
        file_base=$(basename $file)
        new_filepath=$test_dir/$new_output_dir/$file_base.$new_output
        old_filepath=$test_dir/$old_output_dir/$file_base.$old_output

        # Compile and push the output to new_filepath
        $compiler $file -o $program -c -I$stdlib
        $program >> $new_filepath
        
        # Force putting contents of new_output to old_output
        if [ "$force_change" = true ]; then
            cat $new_filepath > $old_filepath
        fi

        # Compare outputs, new_output with old_output
        diff_command="diff $old_filepath $new_filepath"
        diff_output=$($diff_command)
        if [[ -z "$diff_output" ]]; then 
            if [ $silent == false ]; then
                echo "[TEST] Testing: $file"
                echo "[TEST]    Okay: New ($new_filepath) == Old ($old_filepath)"
            fi
        else
            echo "[TEST] Testing: $file"
            echo "[TEST]    Diff: New ($new_filepath) != Old ($old_filepath)"
            if [ "$diff_show" = true ]; then
                echo "[SHOW]:"
                $diff_command --color=always
            fi
        fi
    fi
done
