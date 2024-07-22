# Stack based programming language

Programming language using the Polish reversed syntax.
The compiler generates 'x86_64 intel' assembly code which is then compiled with 'nasm' into executable.

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

Command to compile and run the code.
```sh
./target/debug/stack_based_PL com my-programming-language/04-if.abc
./program
```

Command to simulate the code.
```sh
./target/debug/stack_based_PL sim my-programming-language/03-loops.abc
```