#![allow(unused)]

use anyhow::{anyhow, Result};
use std::collections::LinkedList;

fn main() -> Result<()> {
    Ok(())
}

struct DynamicStack<T> {
    link: LinkedList<T>,
}

impl<T> DynamicStack<T> {
    fn new() -> Self {
        Self {
            link: LinkedList::new(),
        }
    }

    fn push(&mut self, v: T) {
        self.link.push_front(v)
    }

    fn pop(&mut self) -> Option<T> {
        self.link.pop_front()
    }
}

struct Stack<const N: usize, T: Copy> {
    top: usize,
    arr: [T; N],
}

impl<const N: usize, T: Copy> Stack<N, T> {
    fn new(arr: [T; N]) -> Self {
        Self { arr, top: 0 }
    }

    fn push(&mut self, v: T) -> Result<()> {
        if self.top < self.arr.len() {
            self.top += 1;
            self.arr[self.top - 1] = v;
            return Ok(());
        }
        Err(anyhow!(""))
    }

    fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            None
        } else {
            let t = self.arr[self.top - 1];
            self.top -= 1;
            Some(t)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn dynamic_stack_should_work() {
        let mut stack = DynamicStack::new();

        for _ in 0..3 {
            stack.push(1);
            stack.push(2);

            assert_eq!(Some(2), stack.pop());
            assert_eq!(Some(1), stack.pop());
            assert_eq!(None, stack.pop());
        }
    }

    #[test]
    fn stack_should_work() {
        let mut stack = Stack::new([0, 2]);

        for _ in 0..3 {
            assert!(stack.push(1).is_ok());
            assert!(stack.push(2).is_ok());
            assert!(stack.push(3).is_err());

            assert_eq!(Some(2), stack.pop());
            assert_eq!(Some(1), stack.pop());
            assert_eq!(None, stack.pop());
            assert_eq!(None, stack.pop());
        }
    }
}
