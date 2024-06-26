use std::ops::{Range, Deref, DerefMut};
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub struct RangeError {
    pub message: String,
    pub range: Range<usize>
}

impl RangeError {
    pub fn pretty_print(&self, program: &str) {
        let Range { start, end } = self.range;
        println!("\n\x1b[31mError\x1b[0m: {}:{}: {}", start, end, self.message);

        let lines = program.lines();
        let mut line_start = 0;

        for line in lines {
            let line_end = line_start + line.len();
            println!("{line_start} {line_end}");
            println!("{line}");
            if true { println!("{}{}", " ".repeat(start - line_start), "^".repeat(end + 1 - start)); }
            line_start = line_end;
        }
    } 
}

pub trait PositionError<T> {
    fn position(self, range: Range<usize>) -> Result<T, RangeError> where Self: Sized;
}

impl<T> PositionError<T> for Result<T, String> {
    fn position(self, range: Range<usize>) -> Result<T, RangeError> {
        match self {
            Ok(t) => Ok(t),
            Err(message) => Err(RangeError { message, range })
        }
    }
}

pub struct Positioned<T> {
    pub inner: T,
    pub range: Range<usize>
}

pub fn position<T>(inner: T, range: Range<usize>) -> Positioned<T> {
    Positioned { inner, range }
}

impl<T: Clone> Clone for Positioned<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            range: self.range.clone(),
        }
    } 
}

impl<T: Debug> Debug for Positioned<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.inner.fmt(f)
    }
}

impl<T> Deref for Positioned<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> DerefMut for Positioned<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}
