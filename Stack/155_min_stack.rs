use std::{cmp::Ordering, i32::MAX};

struct MinStack {
    stack: Vec<i32>,
    min: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        match self.min.last().unwrap_or_else(|| &MAX).cmp(&val) {
            Ordering::Greater => self.min.push(val.clone()),
            _ => self
                .min
                .push(self.min.last().unwrap_or_else(|| &val).clone()),
        }
        self.stack.push(val);
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.min.pop();
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap() //assuming top is called on non empty stack
    }

    fn get_min(&self) -> i32 {
        *self.min.last().unwrap() //assuming min is called on non empty stack
    }
}
