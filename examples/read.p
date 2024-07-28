macro stderr 0 end
macro stdout 1 end
macro stdin 2 end
macro sys_write 1 end
macro sys_read 0 end
macro sys_exit 60 end
macro NAME_CAP 256 end
macro write
    //size buffer fd
    sys_write syscall3
end
macro read
    //size buffer fd
    sys_read syscall3
end
macro exit 
    //code
    60 syscall1
end
macro name mem end

//program--------------------

"What is your name? " stdout write drop
NAME_CAP name stdin read

dup 0 <= if
    "ERROR: Could not read the name" stderr write drop
    1 exit
else dup 1 <= if
    "Please enter you name!" stdout write drop
    1 exit
end
end

"Hello " stdout write drop
//bytes read returned
1 - mem stdout write drop
"!!!\n" stdout write drop

0 exit