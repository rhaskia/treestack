use std::{ops::{Add, Deref, DerefMut, Mul, Sub}, fmt::Display};

#[derive(Debug, Clone)]
pub struct TreeNode<T> {
    pub val: T,
    pub children: Vec<TreeNode<T>>, 
}

impl<T> TreeNode<T> {
    pub fn push_raw(&mut self, item: T) {
        self.children.push(TreeNode { val: item, children: Vec::new() })
    }
}

impl<T: Mul<Output = T>> Mul for TreeNode<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
       Self { val: self.val * rhs.val, children: self.children } // TODO children mult 
    }
}

impl<T: Add<Output = T>> Add for TreeNode<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
       Self { val: self.val + rhs.val, children: self.children } // TODO children mult 
    }
}

impl<T: Sub<Output = T>> Sub for TreeNode<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
       Self { val: self.val - rhs.val, children: self.children } // TODO children mult 
    }
}

impl<T: Display> Display for TreeNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.val.fmt(f)
    }
}

impl<T: Default> TreeNode<T> {
    pub fn new() -> Self {
        Self { val: T::default(), children: Vec::new() }
    }
}

impl<T> Deref for TreeNode<T> {
    type Target = Vec<TreeNode<T>>;

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl<T> DerefMut for TreeNode<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}
