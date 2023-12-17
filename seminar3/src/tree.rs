pub struct TreeNode<T> {
    val: T,
    children: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            children: vec![],
        }
    }
}
