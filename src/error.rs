use std::ops::Range;

struct Error {
    message: String,
    range: Range<usize>
}
