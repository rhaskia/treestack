declare i32 @puts(i8* nocapture) nounwind

; Define the type for a 32-bit integer

; Define a global variable to store the stack pointer
; This points to the top of the stack (initially empty)
define internal i32* @stack_pointer = alloca (i32* null)

declare void @llvm.eh.terminate() ; (for error handling)

define i32 @push(i32 %value) {
entry:
  ; Get the current stack pointer
  %current_ptr = load i32*, i32** @stack_pointer

  ; Allocate space for a new element on the stack (one integer)
  %new_ptr = call i32* @alloca(i32, i32 4)

  ; Store the value in the new element
  store i32 %value, i32* %new_ptr

  ; Update the stack pointer to point to the new element
  store i32* %new_ptr, i32** @stack_pointer

  ; Return the pushed value (for potential chaining)
  ret i32 %value
}

define i32 @pop() {
entry:
  ; Check if the stack is empty
  %current_ptr = load i32*, i32** @stack_pointer
  %is_null = icmp eq i32* null, i32* %current_ptr
  br i1 %is_null, dead, then

then:
  ; Get the top element from the stack
  %top_ptr = load i32*, i32** @stack_pointer

  ; Load the value from the top element
  %value = load i32, i32* %top_ptr

  ; Get the address of the previous element (new stack pointer)
  %prev_ptr = getelementptr inbounds i32, i32* %top_ptr, i32 -1

  ; Update the stack pointer to point to the previous element
  store i32* %prev_ptr, i32** @stack_pointer

  ; Return the popped value
  ret i32 %value

dead:
  ; Error: Stack underflow
  call void @llvm.eh.terminate()
  unreachable
}

; Example usage of push and pop functions
define i32 @main() {
entry:
  ; Push some values onto the stack
  call i32 @push(i32 5)
  call i32 @push(i32 3)

  ; Pop and add the values
  %value1 = call i32 @pop()
  %value2 = call i32 @pop()
  %result = add i32 %value1, %value2

  ; Print the result (you'll need additional functions for printing)
  ; ... print result ...
  call i32 @puts(i8* @stack_pointer)

  ret i32 0
}

