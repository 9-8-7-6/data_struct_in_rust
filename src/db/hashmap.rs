use std::fmt::Debug;

#[derive(Debug)]
pub struct Hashmap<T> {
    size: usize,
    slot: Vec<usize>,
    data: Vec<T>,
}

impl<T: Clone + Default + PartialEq> Hashmap<T> {
    pub fn new(size: usize) -> Self {
        let mut slot = Vec::with_capacity(size);
        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            slot.push(0);
            data.push(Default::default());
        }

        Hashmap { size, slot, data }
    }

    pub fn hash(&self, key: usize) -> usize {
        key % self.size
    }

    pub fn rehash(&self, pos: usize) -> usize {
        (pos + 1) % self.size
    }

    pub fn insert(&mut self, key: usize, value: T) {
        if 0 == key {
            panic!("Error: key must > 0");
        }

        let pos = self.hash(key);
        if self.slot[pos] == 0 {
            self.slot[pos] = key;
            self.data[pos] = value;
        } else {
            // Find the next position which can be used.
            let mut next = self.rehash(pos);
            while self.slot[next] != 0 && self.slot[next] != key {
                next = self.rehash(next);
                if next == pos {
                    println!("Error: slot is full, quit insertion");
                    return;
                }
            }

            if self.slot[next] == 0 {
                self.slot[next] = key;
                self.data[next] = value;
            } else {
                self.data[next] = value;
            }
        }
    }

    pub fn remove(&mut self, key: usize) -> Option<T> {
        if 0 == key {
            panic!("Error: key must > 0");
        }

        let pos = self.hash(key);
        if self.slot[pos] == 0 {
            None
        } else if self.slot[pos] == key {
            self.slot[pos] = 0;
            let data = Some(self.data[pos].clone());
            self.data[pos] = Default::default();
            data
        } else {
            let mut data: Option<T> = None;
            let mut stop = false;
            let mut found = false;
            let mut curr = pos;

            while 0 != self.slot[curr] && !found && !stop {
                if self.slot[curr] == key {
                    found = true;
                    self.slot[curr] = 0;
                    data = Some(self.data[curr].clone());
                    self.data[curr] = Default::default();
                } else {
                    curr = self.rehash(curr);
                    if curr == pos {
                        stop = true;
                    }
                }
            }
            data
        }
    }

    pub fn get(&self, key: usize) -> Option<&T> {
        if 0 == key {
            panic!("Error: key must > 0");
        }

        let pos = self.hash(key);
        let mut data: Option<&T> = None;
        let mut stop = false;
        let mut found = false;
        let mut curr = pos;

        while self.slot[curr] != 0 && !found && !stop {
            if self.slot[curr] == key {
                found = true;
                data = self.data.get(curr);
            } else {
                curr = self.rehash(curr);
                if curr == pos {
                    stop = true;
                }
            }
        }

        data
    }

    pub fn contains(&self, key: usize) -> bool {
        if 0 == key {
            panic!("Error: key must > 0");
        }
        self.slot.contains(&key)
    }

    pub fn len(&self) -> usize {
        let mut length = 0;
        for d in self.slot.iter() {
            if d != &0 {
                length += 1;
            }
        }
        length
    }
}
