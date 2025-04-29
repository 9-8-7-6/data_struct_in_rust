#[derive(Debug)]
pub struct Deque<T> {
    len: usize,
    data: Vec<T>,
}

impl<T> Deque<T> {
    pub fn new(size: usize) -> Self {
        Deque {
            len: size,
            data: Vec::with_capacity(size),
        }
    }

    pub fn push_front(&mut self, item: T) -> Result<(), String> {
        if Self::size(&self) == self.len {
            return Err("No space available".to_string());
        }
        self.data.insert(0, item);

        Ok(())
    }

    pub fn push_back(&mut self, item: T) -> Result<(), String> {
        if Self::size(&self) == self.len {
            return Err("No space available".to_string());
        }
        self.data.push(item);

        Ok(())
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            Some(self.data.remove(0))
        } else {
            None
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    pub fn is_empty(&mut self) -> bool {
        Self::size(&self) == 0
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}
