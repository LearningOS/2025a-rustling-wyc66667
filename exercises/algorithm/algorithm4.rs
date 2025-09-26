/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//I AM NOT DONE
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
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        // 填补开始：BST的插入逻辑
        // 如果根节点为空，直接创建新节点作为根
        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode::new(value)));
        } else {
            // 否则调用根节点的insert方法递归插入
            self.root.as_mut().unwrap().insert(value);
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        // 填补开始：BST的查找逻辑
        // 从根节点开始查找
        let mut current = &self.root;
        
        // 遍历树直到找到节点或到达叶子节点
        while let Some(node) = current {
            match node.value.cmp(&value) {
                Ordering::Equal => return true,  // 找到匹配值
                Ordering::Greater => current = &node.left,  // 向左子树查找
                Ordering::Less => current = &node.right,    // 向右子树查找
            }
        }
        
        false  // 未找到
        // 填补结束
        
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        // 填补开始：节点的插入逻辑
        match self.value.cmp(&value) {
            // 相等的值不插入（BST通常不允许重复值）
            Ordering::Equal => return,
            
            // 新值小于当前节点值，插入左子树
            Ordering::Greater => {
                if self.left.is_none() {
                    // 左子树为空，直接创建新节点
                    self.left = Some(Box::new(TreeNode::new(value)));
                } else {
                    // 左子树不为空，递归插入
                    self.left.as_mut().unwrap().insert(value);
                }
            }
            
            // 新值大于当前节点值，插入右子树
            Ordering::Less => {
                if self.right.is_none() {
                    // 右子树为空，直接创建新节点
                    self.right = Some(Box::new(TreeNode::new(value)));
                } else {
                    // 右子树不为空，递归插入
                    self.right.as_mut().unwrap().insert(value);
                }
            }
        }
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
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


