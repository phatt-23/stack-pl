
macro [[]] 1 syscall3 end

macro (()) 1 [[]] end       // will get expended to 1 1 syscall3

macro write (()) end        // alias of (())

"Hello, World\n" (()) "Fooooooo Baar\n" write

macro PAGE_SIZE 1584 end

PAGE_SIZE dup + dump