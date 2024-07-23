
macro A 1234 end

1 3 < if
    A A = if
        12 23 > if
            12345 dump
        end
    else
        54321 dump
    end
    123 dump // <--- only this is printed 
else
    321 dump
end


1 1 > if
    111 dump
else 
    1 1 < if
        222 dump
    else 
        1 1 = if
            333 dump // <---- this is printed
        else
            444 dump
        end
    end
end
