use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Heap<T> {
    items: HashMap<usize, T>,
}

impl<T> Heap<T> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn push(&mut self, item: T) -> usize {
        // I know this is a really dumb implementation
        let mut key = self.items.len();

        while self.items.contains_key(&key) {
            key += 1;
        }

        self.items.insert(key, item);

        key
    }

    pub fn filter_keys(&mut self, to_keep: &[usize]) {
        self.items.retain(|k, _| to_keep.contains(k))
    }

    pub fn get_mut<'a>(&'a mut self, key: &usize) -> Option<&'a mut T> {
        self.items.get_mut(key)
    }

    pub fn get<'a>(&'a self, key: &usize) -> Option<&'a T> {
        self.items.get(key)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
