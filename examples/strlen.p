include "stdlib.p"

macro strlen
  0 swap while dup , 0 != do
    1 + swap 1 + swap
  end drop
end

"Hello, world\0" strlen print print
