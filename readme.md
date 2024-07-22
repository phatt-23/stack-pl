# Stack based programming language

# About

Programming language using the Polish reversed syntax.
The compiler generates 'x86_64 intel' assembly code which is then compiled with 'nasm' into executable.

# Features

- [x] Arithmetic Operators
- [x] Logic Operators
- [x] Bitwise Operators
- [x] If-Else Statement
- [x] Loops (while)
- [x] Memory Access
- [x] String Literal
- [x] Syscall
- [ ] Macros
- [ ] Types

# Usage 

```sh
./target/debug/stack_based_PL --help
```

# Stages of compilation

```
  (Source File) => lexer => [Tokens] => 
```

1) Lexer: (lexical analysis)
    - parses through the source file,
    - converts each word seprates by whitespaces into a list of tokens
    Each tokens holds information about:
    - location it ocupies within the source file, 
    - token type tells if its a Word, Interger or Str (only these are supported now),
    - token value, based on the token type, holds the actual value of the token (Word, String) (Integer, i32) (Str, String)

Lexing stage _does not provide meaning_ to the program, it merely 
tokenizes/converts words in a file to a format that is more friendly to then _analyse_ later.

2) Analyzer: (syntax and semantic analysis)
    - takes in tokens and assign them meaning, effectively converting _tokens_ to _operations_
    - it iterates through all the tokens and map them to operations (e.g. Word '+' gets converted into an Addition operation)
    - macros get expanded - each time the program uses them by refering their name, it inlines their bodies (note: they are not real operations, by the time of code generation stage they are completely discarded)
    - the operations that form blocks (if else, while do) get crossreferenced
    - simple operations will simply be mapped - e.g. token (Word, ">>") will be mapped to operation ShiftRight and so on

3) Generator: (code generation)
    - the operations convert to assembly code by simple mapping from an operation to a series of inscructions
    - the resulting assembly code is compiled by assembly compiler 'nasm' and linked against 'ld' (for now only for linux-nasm-x86_64)
