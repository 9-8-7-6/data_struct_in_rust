/*
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn top(&mut self) -> Option<&T> {
        self.items.last()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn get_size(&self) -> usize {
        self.items.len()
    }
}
*/

use std::alloc::{alloc, dealloc, realloc, Layout};
use std::mem::MaybeUninit;
use std::ptr::{self, NonNull};

const STACK_INIT_SIZE: usize = 10;
const STACK_INCREMENT: usize = 2;

pub struct Stack<T> {
    base: NonNull<MaybeUninit<T>>,
    top: *mut MaybeUninit<T>,
    capacity: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        let layout = Layout::array::<MaybeUninit<T>>(STACK_INIT_SIZE).unwrap();
        let ptr = unsafe { alloc(layout) as *mut MaybeUninit<T> };

        let base = NonNull::new(ptr).expect("Failed to allocate memory");
        let top = ptr;

        Self {
            base,
            top,
            capacity: STACK_INIT_SIZE,
        }
    }

    pub fn len(&self) -> usize {
        unsafe { self.top.offset_from(self.base.as_ptr()) as usize }
    }

    fn layout(&self) -> Layout {
        Layout::array::<MaybeUninit<T>>(self.capacity).unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        unsafe {
            while self.top > self.base.as_ptr() {
                self.top = self.top.offset(-1);
                ptr::drop_in_place(self.top as *mut T);
            }
        }
    }

    pub fn push(&mut self, val: T) {
        if self.len() == self.capacity {
            self.grow();
        }

        unsafe {
            (*self.top).write(val);
            self.top = self.top.add(1);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        unsafe {
            self.top = self.top.offset(-1);
            Some(self.top.read().assume_init())
        }
    }

    pub fn top(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        unsafe { Some((&*self.top.offset(-1)).assume_init_ref()) }
    }

    pub fn traverse<F: FnMut(&T)>(&self, mut visit: F) {
        let mut ptr = self.base.as_ptr();
        while ptr < self.top {
            unsafe {
                visit(ptr.read().assume_init_ref());
            }
            ptr = unsafe { ptr.add(1) };
        }
    }

    fn grow(&mut self) {
        let new_capacity = self.capacity + STACK_INCREMENT;
        let new_layout = Layout::array::<MaybeUninit<T>>(new_capacity).unwrap();

        unsafe {
            let new_ptr = realloc(
                self.base.as_ptr() as *mut u8,
                self.layout(),
                new_layout.size(),
            ) as *mut MaybeUninit<T>;

            if new_ptr.is_null() {
                panic!("Realloc failed");
            }

            let top_offset = self.top.offset_from(self.base.as_ptr());

            self.base = NonNull::new(new_ptr).unwrap();
            self.top = new_ptr.offset(top_offset);
            self.capacity = new_capacity;
        }
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        self.clear();

        unsafe {
            dealloc(self.base.as_ptr() as *mut u8, self.layout());
        }
    }
}
