# Stack based programming language

## About

Programming language using the Polish reversed syntax.

The compiler generates 'x86_64 intel' assembly code which is then compiled with 'nasm' into executable.

## Features

- [x] Arithmetic Operators
- [x] Logic Operators
- [x] Bitwise Operators
- [x] If-Else Statement
- [x] Loops (while)
- [x] Memory Access
- [x] String Literal
- [x] Syscall
- [x] Macros
- [x] Include Files

## Usage 

```sh
./target/debug/stack_based_PL --help
```

## Stages of compilation

```hs
# The compiler pipeline
(Source File) 
    => lexer => [Tokens] 
        => analyser => [Operations] 
            => generator => (Assembly File) 
                ~> compiled with 'nasm' and linked with 'ld'
```

### Lexer: (lexical analysis)

- parses through the source file,
- converts each word seprates by whitespaces into a list of tokens

Each tokens holds information about:
- `location` it ocupies within the source file, 
- `token type` tells if its a `Word`, `Integer` or `Str` (only these are supported now),
- `token value`, based on the `token type`, holds the actual value of the token 
    - `Word` - String
        - `Integer` - i32
        - `Str` - String

Lexing stage _does not provide meaning_ to the program, it merely 
tokenizes/converts words in a file to a format that is more friendly to then _analyse_ later in the next stage.

### Analyzer: (syntax and semantic analysis)
- takes in tokens and assign them meaning, effectively converting _tokens_ to _operations_ - it iterates through all the tokens and converts them to operations
- `macros` get expanded
    - each time the program uses them by refering their name, it inlines their bodies 
    - note that they are not real operations, by the time of code generation stage they themselves are completely discarded
- __block operations__, `if else`, `while do`, get crossreferenced - their jump address is set by finding pairs of block _start_ and _end_
- __simple operations__ will simply be mapped:
    - `Word` `"+"` maps to `Addition` operation,
    - `Word` `"dump"` maps to `Dump` operation,
    - `Integer` `123` maps to `PushInt` operation with value `123`
    - `Str`, `"Hello, World\n"` maps to `PushStr` with value `"Hello, World\n"`

### Generator: (code generation)
- the operations convert to assembly code by direct mapping from an operation to a series of inscructions
- string get allocated in `section .data`
- the resulting assembly code is compiled with `nasm` assembly compiler  and linked against witch `ld` (for now only for linux_nasm_x86_64)