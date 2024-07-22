
macro (()) 1 1 syscall3 end
    
macro write 2 1 syscall3 end

"Hello, World\n" (()) "Fooooooo Baar\n" write

macro PAGE_SIZE 1584 end

PAGE_SIZE dup + dump