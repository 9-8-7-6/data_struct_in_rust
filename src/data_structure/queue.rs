#[derive(Debug)]
pub struct Queue<T> {
    len: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        Queue {
            len: size,
            data: Vec::with_capacity(size),
        }
    }

    pub fn enqueue(&mut self, item: T) -> Result<(), String> {
        if Self::size(&self) == self.len {
            return Err("No space available".to_string());
        }
        self.data.insert(0, item);

        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<T> {
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
