// stdio.h

macro SYS_WRITE 1 end
macro STDOUT 1 end
macro write STDOUT SYS_WRITE syscall3 end
