use std::fmt::Display;

pub struct FoundIdent<T> {
    pub value: T,
    pub index: usize,
}

#[derive(Clone)]
pub struct PoppedStack<T> {
    values: Vec<(String, T)>,
    frames: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Stack<T>
where
    T: Clone,
{
    values: Vec<(String, T)>,
    /// Each item is start of each stack frame
    frames: Vec<usize>,
}

impl<T> Stack<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            frames: vec![0],
        }
    }

    pub fn pop_frame(&mut self) -> Option<Vec<(String, T)>> {
        let frame = self.frames.pop()?;

        Some(self.values.split_off(frame))
    }

    pub fn open_frame(&mut self) {
        self.frames.push(self.values.len())
    }

    pub fn push_frame(&mut self, mut values: Vec<(String, T)>) {
        self.frames.push(self.values.len());
        self.values.append(&mut values);
    }

    pub fn push_value(&mut self, ident: String, value: T) {
        self.values.push((ident, value))
    }

    pub fn value_len(&self) -> usize {
        self.values.len()
    }

    pub fn frame_len(&self) -> usize {
        self.frames.len()
    }

    /// Pop all elements after specific index
    pub fn pop_until_index(&mut self, index: usize) -> PoppedStack<T> {
        let values = self.values.split_off(index + 1);

        let containing_frame = self
            .frames
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, f)| (*f <= index).then_some(i))
            .unwrap();

        let frames = self.frames.split_off(containing_frame + 1);

        PoppedStack { values, frames }
    }

    pub fn push_popped_stack(&mut self, popped: PoppedStack<T>) {
        let PoppedStack {
            mut values,
            mut frames,
        } = popped;
        self.values.append(&mut values);
        self.frames.append(&mut frames);
    }

    pub fn find_with_ident<'a>(&'a self, ident: &str) -> Option<FoundIdent<T>> {
        let (index, value) = self
            .values
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, s)| s.0.eq(ident).then_some((index, s.1.clone())))?;

        Some(FoundIdent { value, index })
    }

    pub fn iter_values(&'_ self) -> impl Iterator<Item = T> + '_ {
        self.values.iter().map(|(_, value)| value.clone())
    }
}

impl<T> Display for Stack<T>
where
    T: Display + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current_frame = 0;

        for i in 0..self.values.len() {
            if let Some(next_frame_start) = self.frames.get(current_frame + 1) {
                if i > *next_frame_start {
                    current_frame += 1;
                    writeln!(f, "FRAME: {current_frame}")?;
                }
            }

            writeln!(f, "\t{}: {}", self.values[i].0, self.values[i].1)?;
        }

        Ok(())
    }
}
