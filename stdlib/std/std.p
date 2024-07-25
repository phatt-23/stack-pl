// standard library

macro + add end
macro - sub end
macro * mul end
macro / divmod drop end
macro % divmod swap drop end

macro write 1 1 syscall3 end

macro mem memp end
macro . mems end
macro , meml end

macro < le end
macro > gr end
macro <= leeq  end
macro >= greq  end
macro = eq  end
macro != neq  end
macro ! not end

macro << shl end 
macro >> shr end
macro | bor end
macro & band end

macro dup2 over over end

