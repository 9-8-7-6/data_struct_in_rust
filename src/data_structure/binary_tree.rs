use std::fmt::Debug;

type Node<T> = Option<Box<BinaryTree<T>>>;

#[derive(Debug, Clone)]
pub struct BinaryTree<T> {
    pub key: T,
    pub left: Node<T>,
    pub right: Node<T>,
}

impl<T: Clone> BinaryTree<T> {
    pub fn new(key: T) -> Self {
        BinaryTree {
            key: key,
            left: None,
            right: None,
        }
    }

    pub fn insert_left_tree(&mut self, key: T) {
        if self.left.is_none() {
            let node = BinaryTree::new(key);
            self.left = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.left = self.left.take();
            self.left = Some(Box::new(node));
        }
    }

    pub fn insert_right_tree(&mut self, key: T) {
        if self.right.is_none() {
            let node = BinaryTree::new(key);
            self.right = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.right = self.right.take();
            self.right = Some(Box::new(node));
        }
    }

    pub fn get_left(&self) -> Node<T> {
        self.left.clone()
    }

    pub fn get_right(&self) -> Node<T> {
        self.right.clone()
    }

    pub fn get_key(&self) -> T {
        self.key.clone()
    }

    pub fn set_key(&mut self, key: T) {
        self.key = key;
    }
}
