// standard library

macro / divmod drop end
macro % divmod swap drop end

macro write 1 1 syscall3 end

macro mem memp end
macro . mems end
macro , meml end

macro << shl end 
macro >> shr end
macro | or end
macro & and end

macro dup2 over over end

