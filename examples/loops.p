include "stdlib.p"

"fibonacci------------------\n" write drop
0 1
while dup2 10000000000000000 < do 
    +
    over print 
    swap
end drop
"---------------------------\n" write drop
0 
0 while dup2 5 < do 
    over + dup print swap
    1 +
end drop
"---------------------------\n" write drop
0 while dup 10 < do
    dup print
    1 +
end drop
"---------------------------\n" write drop
10 while dup 0 > do
    dup print
    1 -
end drop
