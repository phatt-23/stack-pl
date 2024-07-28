include "stdlib.p"

mem dup 72  !8  
1 + dup 101 !8 
1 + dup 108 !8 
1 + dup 108 !8 
1 + dup 111 !8 
1 + dup 10  !8 
1 +

dup mem - mem 1 1 syscall3

// mem dump // <---- this will be different in com and sim

0
0
print print

123 60 syscall1


