# Treestack
"If you're over 25 and own a computer this language is a must-try!"
(Insert tree picture)
Treestack works essentially like a regular stack language, except each stack contains a stack of their own.
Treestack also allows for more movement around the stack/s than a Forth for example, and also allows for pointers to locations on the stack.

# Syntax

## Functions
A function is essentially just a group of statements, and does not allow for any parameters (maybe allow for pointers in future)
; NAME + * ^ ; 
This constructs a function called NAME, that calls the operators + * and ^ on the current position on the stack.

## Pointers
&name creates a pointer called name at the current position
*name pushes the value at the pointer onto the stack
^name jumps to the pointer's location

## Basic Words
