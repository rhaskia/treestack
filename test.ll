%Node = type {
    i64
}

declare i32 @puts(i8* nocapture) nounwind

@.hello = private unnamed_addr constant [13 x i8] c"hello world\0A\00"

define void @main() {
    %bar = alloca %Node, i32 100
    %1 = getelementptr %Node, %Node* %bar, i32 17, i32 0
    store i64 70, i64* %1
    %stack = add i32 0, 1

    %2 = getelementptr [13 x i8], [13 x i8]* @.hello, i32 0, i32 0
    %b = alloca i8, i32 4
    %3 = getelementptr i8, i8* %b, i32 0 
    %4 = getelementptr i8, i8* %b, i32 1
    %5 = getelementptr i8, i8* %b, i32 2
    store i8 65, i8* %3 
    store i8 66, i8* %4
    store i8 66, i8* %5

    call i32 @puts(i8* %b)
    call i32 @puts(i8* %2)

    ret void
}
