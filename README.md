"If you're over 25 and own a computer this language is a must-try!"
(Insert wise mystical tree picture)
Treestack works essentially like a regular stack language, except each stack contains a stack of their own.
Treestack also allows for more movement around the stack/s than a Forth for example, and also allows for pointers to locations on the stack.

Check out some of the examples in the ./examples directory, any of them can be run with `treestack file` if you have it compiled, or just `cargo run -- file`.
Debug is also available
# Syntax
The syntax is very simple, and is very close to forth with its reverse polish packer notation. It is is derived of 3-4 main types: [operators](operators.md), [words](words.md), and pushing; through raw number literals, string literals and char literals. There is also some control flow that acts a bit differently, which is described in [words](words.md).
# Ideas
bool types
ast rotation ( split off into ast sections of string )
