// standard library

macro / divmod drop end
macro % divmod swap drop end

macro write 1 1 syscall3 end

macro << shl end 
macro >> shr end
macro | or end
macro & and end

macro dup2 over over end

macro strlen
  0 swap while dup *8 0 != do
    1 + swap 1 + swap
  end drop
end

