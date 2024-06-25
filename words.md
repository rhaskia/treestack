dup (n -- n n)
swap (a b -- b a)
over (a b -- a b a)

# File IO
read ((file) -- (file_contents))
write ((file) (to write) -- writes to a file)
syscall (sysno -- runs a syscall)

# Other
print ((string) -- prints string)
random ( -- random value )

# Control Flow
if { expr } else { expr }
while { expr }
if and while work off the stack value being "truthy", or not 0

fn function_name { expr }
allows for creating words from expressions
