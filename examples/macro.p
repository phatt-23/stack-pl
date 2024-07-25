include "std.p"

macro [[]] 1 syscall3 end
macro (()) 1 [[]] end       
macro my_write (()) end        

"Hello, World\n" (()) 
"Fooooooo Baar\n" my_write

macro MY_SIZE 1584 end

MY_SIZE dup + dump

