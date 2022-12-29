use std::alloc::{alloc, dealloc, Layout};
use std::ptr::write;

/// We wrap pointers so they can't be misused outside this module
///
/// NOTE: The [`PartialEq`] implementation checks pointer locations, not content.
/// NOTE: The [`Hash`] implementation only looks at pointer locations, not content.
/// NOTE: The [`Clone`] and [`Copy`] implementations do so by referance, not content.
#[derive(Debug)]
pub struct HeapItem<T> {
    pub(self) inner: *mut T,
}

impl<T> PartialEq for HeapItem<T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.inner, other.inner)
    }
}

impl<T> Eq for HeapItem<T> {}

impl<T> Clone for HeapItem<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner
        }
    }
}

impl<T> Copy for HeapItem<T> {}

impl<T> HeapItem<T> {
    /// Create a new HeapItem from a pointer
    fn new(inner: *mut T) -> Self {
        Self { inner }
    }
}

impl<T> std::hash::Hash for HeapItem<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.inner as usize);
    }
}

#[derive(Debug, Clone)]
pub struct Heap<T> {
    allocated: Vec<HeapItem<T>>,
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
            self.allocated.push(HeapItem::new(ptr));
            HeapItem::new(ptr)
        }
    }

    pub fn filter_keys(&mut self, to_keep: &[HeapItem<T>]) {
        self.allocated.retain(|a| {
            if !to_keep.contains(a) {
                unsafe {
                    a.inner.drop_in_place();
                    dealloc(a.inner.cast::<u8>(), Layout::new::<T>());
                    false
                }
            } else {
                true
            }
        });
    }

    pub fn set(&mut self, key: HeapItem<T>, new_value: T) {
        if !cfg!(debug_assertions) || self.allocated.contains(&key) {
            unsafe { *key.inner = new_value }
        } else {
            panic!("HEAP DOES NOT CONTAIN POINTER")
        }
    }

    pub fn get_mut<'a>(&'a mut self, key: HeapItem<T>) -> &'a mut T {
        if !cfg!(debug_assertions) || self.allocated.contains(&key) {
            unsafe { &mut *key.inner }
        } else {
            panic!("HEAP DOES NOT CONTAIN POINTER")
        }
    }

    pub fn get<'a>(&'a self, key: HeapItem<T>) -> &'a T {
        if !cfg!(debug_assertions) || self.allocated.contains(&key) {
            unsafe { &mut *key.inner }
        } else {
            panic!("HEAP DOES NOT CONTAIN POINTER")
        }
    }

    pub fn len(&self) -> usize {
        self.allocated.len()
    }
}
