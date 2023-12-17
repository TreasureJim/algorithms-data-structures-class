pub struct BinaryTreeNode<T> {
    pub val: T,
    pub left: Option<Box<BinaryTreeNode<T>>>,
    pub right: Option<Box<BinaryTreeNode<T>>>,
}

impl<T> BinaryTreeNode<T> {
    pub fn new(val: T) -> Self {
        Self { val, left: None, right: None }
    }
}
