use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // 辅助方法：向当前节点的子树中插入值
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            // 值小于当前节点，插入左子树
            Ordering::Less => {
                if let Some(left) = &mut self.left {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            // 值大于当前节点，插入右子树
            Ordering::Greater => {
                if let Some(right) = &mut self.right {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            // 相等的值不插入（BST通常不允许重复值）
            Ordering::Equal => (),
        }
    }

    // 辅助方法：在当前节点的子树中搜索值
    fn search(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Equal => true,  // 找到匹配值
            Ordering::Less => self.left.as_ref().map_or(false, |left| left.search(value)),  // 搜索左子树
            Ordering::Greater => self.right.as_ref().map_or(false, |right| right.search(value)),  // 搜索右子树
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // 向BST中插入值
    fn insert(&mut self, value: T) {
        if let Some(root) = &mut self.root {
            root.insert(value);
        } else {
            // 如果树为空，创建新节点作为根
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    // 在BST中搜索值
    fn search(&self, value: T) -> bool {
        self.root.as_ref().map_or(false, |root| root.search(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();
        assert_eq!(bst.search(1), false);
        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();
        bst.insert(1);
        bst.insert(1);  // 插入重复值
        
        assert_eq!(bst.search(1), true);
        
        match bst.root {
            Some(ref node) => {
                // 重复插入后不应创建新节点
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}