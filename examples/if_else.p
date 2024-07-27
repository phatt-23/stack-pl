include "stdlib.p"

macro A 1234 end

1 3 < if
    A A = if
        12 23 > if
            12345 print
        end
    else
        54321 print
    end
    123 print // <--- only this is printed 
else
    321 print
end


1 1 > if
    111 print
else 
    1 1 < if
        222 print
    else 
        1 1 = if
            333 print // <---- this is printed
        else
            444 print
        end
    end
end
