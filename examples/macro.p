include "stdlib.p"

macro [[]] 1 syscall3 end
macro (()) 1 [[]] end
macro my_write (()) end

"Hello, World\n" (()) 
"Fooooooo Baar\n" my_write

macro MY_SIZE 1584 end

MY_SIZE dup + print

macro recursive
  345 print
  recursive
end

//recursive

macro foo
  bar
end

macro bar
  foo
end

//foo

macro max2
  dup2 < if
    "is bigger\n" write
  else
    "is smaller\n" write
  end
end

234 123 max2


macro mac-while
  0 while dup 10 < do
    dup 5 > if
      "greater than 5: " write
      dup print
    else
      "less than 5: " write
      dup print
    end
    1 +
  end drop
end

mac-while
