use std::ops::{Range, Deref, DerefMut};
use std::fmt::{Debug, Formatter};

pub struct Error {
    pub message: String,
    pub range: Range<usize>
}

pub struct Positioned<T> {
    pub inner: T,
    pub range: Range<usize>
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
