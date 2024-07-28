include "stdlib.p"

macro N 100 end

mem N 2 - + 1 !8

0 while dup N 2 - < do
    0 while dup N < do
        dup mem + *8 if
            dup mem + N + '*' !8
        else
            dup mem + N + ' ' !8
        end
        1 +
    end
    mem + N + 10 !8
    N 1 + mem N + 1 1 syscall3 drop

    mem *8 1 <<
    mem 1 + *8 
    |

    1 while dup N 2 - < do
        swap 1 << 7 &
        over mem + 1 + *8 |
        dup2 110 swap >> 1 &
        swap mem + swap !8
        swap

        1 +
    end drop 
    drop

    1 +
end 
drop
