use std::fmt::{Display, Formatter};

use crate::Value;

pub struct FoundIdent<'a> {
    pub value: &'a mut Value,
    pub index: usize,
}

#[derive(Clone)]
pub struct PoppedStack {
    values: Vec<(String, Value)>,
    frames: Vec<usize>,
}

#[derive(Clone)]
pub struct Stack {
    values: Vec<(String, Value)>,
    /// Each item is start of each stack frame
    frames: Vec<usize>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            frames: vec![0],
        }
    }

    pub fn pop_frame(&mut self) -> Option<Vec<(String, Value)>> {
        let frame = self.frames.pop()?;

        Some(self.values.split_off(frame))
    }

    pub fn open_frame(&mut self) {
        self.frames.push(self.values.len())
    }

    pub fn push_frame(&mut self, mut values: Vec<(String, Value)>) {
        self.frames.push(self.values.len());
        self.values.append(&mut values);
    }

    pub fn push_value(&mut self, ident: String, value: Value) {
        self.values.push((ident, value))
    }

    /// Pop all elements after specific index
    pub fn pop_until_index(&mut self, index: usize) -> PoppedStack {
        let values = self.values.split_off(index + 1);

        let containing_frame = self
            .frames
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, f)| (*f <= index).then(|| i))
            .unwrap();

        let frames = self.frames.split_off(containing_frame + 1);

        PoppedStack { values, frames }
    }

    pub fn push_popped_stack(&mut self, popped: PoppedStack) {
        let PoppedStack {
            mut values,
            mut frames,
        } = popped;
        self.values.append(&mut values);
        self.frames.append(&mut frames);
    }

    pub fn find_with_ident<'a>(&'a mut self, ident: &str) -> Option<FoundIdent<'a>> {
        let (index, value) = self
            .values
            .iter_mut()
            .enumerate()
            .rev()
            .find_map(|(index, s)| s.0.eq(ident).then(|| (index, &mut s.1)))?;

        Some(FoundIdent { value, index })
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (frame, value) in self.frames.iter().zip(&self.values) {
            writeln!(f, "{frame}\t{} = {}", value.0, value.1)?;
        }

        Ok(())
    }
}
