# Stack based programming language and compiler

Programming language using the Polish reversed syntax.
The compiler generates 'x86_64 intel' assembly code which is then compiled with 'nasm' into executable.

- [x] Arithmetic operators
- [x] Logic operators
- [x] If-Else statements
- [x] Loops (while loop)
- [ ] Memory Access
- [ ] Types

Command to compile and run the code.
```sh
./target/debug/stack_based_PL com my-programming-language/04-if.abc
./program
```

Command to simulate the code.
```sh
./target/debug/stack_based_PL sim my-programming-language/03-loops.abc
```