include "stdlib.p"

macro N 100 end

mem N 2 - + 1 .

0 while dup N 2 - < do
    0 while dup N < do
        dup mem + , if
            dup mem + N + '*' .
        else
            dup mem + N + ' ' .
        end
        1 +
    end
    mem + N + 10 .
    N 1 + mem N + 1 1 syscall3

    mem , 1 <<
    mem 1 + , 
    |

    1 while dup N 2 - < do
        swap 1 << 7 &
        over mem + 1 + , |
        dup2 110 swap >> 1 &
        swap mem + swap .
        swap

        1 +
    end drop 
    drop

    1 +
end 
drop
