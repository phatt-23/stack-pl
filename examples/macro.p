include "stdlib.p"

macro [[]] 1 syscall3 end
macro (()) 1 [[]] drop end
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
    dup print "is bigger\n" write drop
  else
    dup print "is smaller\n" write drop
  end
end

234 
123 
max2

0 while dup 10 < do
  dup print
  dup 4 < if
    "is smaller than 4\n" write drop
  end 
  1 +
end drop

macro mac-while
  0 while dup 10 < do
    dup 5 > if
      "greater than 5: " write drop
      dup print
    else
      "less than 5: " write drop
      dup print
    end
    1 +
  end drop
end

mac-while
