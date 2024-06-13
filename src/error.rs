use std::ops::{Range, Deref, DerefMut};

struct Error {
    message: String,
    range: Range<usize>
}


struct Positioned<T> {
    inner: T,
    range: Range<usize>
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
