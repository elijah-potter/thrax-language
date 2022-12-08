use std::alloc::{alloc, dealloc, Layout};
use std::collections::{HashMap, HashSet};
use std::ptr::write;

pub type HeapItem<T> = *mut T;

#[derive(Debug, Clone)]
pub struct Heap<T> {
    allocated: Vec<*mut T>,
}

impl<T> Heap<T> {
    pub fn new() -> Self {
        Self {
            allocated: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) -> HeapItem<T> {
        unsafe {
            let ptr = alloc(Layout::new::<T>()).cast::<T>();
            write(ptr, item);
            self.allocated.push(ptr);
            ptr
        }
    }

    pub fn filter_keys(&mut self, to_keep: &[HeapItem<T>]) {
        self.allocated.retain(|a| {
            if !to_keep.contains(a) {
                unsafe {
                    a.drop_in_place();
                    dealloc(a.cast::<u8>(), Layout::new::<T>());
                    false
                }
            } else {
                true
            }
        });
    }

    pub fn get_mut<'a>(&'a mut self, key: &HeapItem<T>) -> &'a mut T {
        if self.allocated.contains(&key) {
            unsafe { &mut **key }
        } else {
            panic!("HEAP DOES NOT CONTAIN POINTER")
        }
    }

    pub fn get<'a>(&'a self, key: &HeapItem<T>) -> &'a T {
        if self.allocated.contains(&key) {
            unsafe { &mut **key }
        } else {
            panic!("HEAP DOES NOT CONTAIN POINTER")
        }
    }

    pub fn len(&self) -> usize {
        self.allocated.len()
    }
}
