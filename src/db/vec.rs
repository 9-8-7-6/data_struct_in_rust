use std::fmt::Debug;

struct Node<T> {
    element: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    pub fn new(element: T) -> Node<T> {
        Node {
            element,
            next: None,
        }
    }
}

pub struct LVec<T> {
    size: usize,
    head: Link<T>,
}

impl<T: Copy + Debug> LVec<T> {
    pub fn new() -> Self {
        LVec {
            size: 0,
            head: None,
        }
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.head = None;
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, element: T) {
        let node = Node::new(element);

        if self.is_empty() {
            self.head = Some(Box::new(node));
        } else {
            let mut curr = self.head.as_mut().unwrap();

            for _i in 0..self.size - 1 {
                curr = curr.next.as_mut().unwrap();
            }
            curr.next = Some(Box::new(node));
        }
        self.size += 1;
    }

    pub fn append(&mut self, other: &mut Self) {
        while let Some(node) = other.head.as_mut().take() {
            self.push(node.element);
            other.head = node.next.take();
        }
        other.clear();
    }

    pub fn insert(&mut self, mut index: usize, element: T) {
        if index >= self.size {
            index = self.size;
        }

        let mut node = Node::new(element);
        if self.is_empty() {
            self.head = Some(Box::new(node));
        } else if index == 0 {
            // 插 入 鏈 表 首 部
            node.next = self.head.take();
            self.head = Some(Box::new(node));
        } else {
            // 插 入 鏈 表 中 間
            let mut curr = self.head.as_mut().unwrap();
            for _i in 0..index - 1 {
                // 找 到 插 入 位 置
                curr = curr.next.as_mut().unwrap();
            }
            node.next = curr.next.take();
            curr.next = Some(Box::new(node));
        }
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.remove(self.size - 1)
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }

        let mut node;
        if index == 0 {
            node = self.head.take().unwrap();
            self.head = node.next.take();
        } else {
            let mut curr = self.head.as_mut().unwrap();
            for _i in 0..index - 1 {
                curr = curr.next.as_mut().unwrap();
            }
            node = curr.next.take().unwrap();
            curr.next = node.next.take();
        }
        self.size -= 1;

        Some(node.element)
    }
}
