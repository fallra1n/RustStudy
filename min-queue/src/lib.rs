#![forbid(unsafe_code)]

use std::cmp;
use std::collections::VecDeque;

#[derive(Default)]
pub struct MinQueue<T> {
    push_stack: VecDeque<(T, T)>,
    pop_stack: VecDeque<(T, T)>,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        Self {
            push_stack: VecDeque::new(),
            pop_stack: VecDeque::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        let min: T = if self.push_stack.is_empty() {
            val.clone()
        } else {
            cmp::min(val.clone(), self.push_stack.back().unwrap().clone().1)
        };

        self.push_stack.push_back((val, min))
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.pop_stack.is_empty() {
            while !self.push_stack.is_empty() {
                let tmp = self.push_stack.pop_back().unwrap().0;

                let min: T = if self.pop_stack.is_empty() {
                    tmp.clone()
                } else {
                    cmp::min(tmp.clone(), self.pop_stack.back().unwrap().clone().1)
                };

                self.pop_stack.push_back((tmp, min));
            }
        }

        if self.pop_stack.is_empty() {
            return None;
        }

        Some(self.pop_stack.pop_back().unwrap().0)
    }

    pub fn front(&self) -> Option<&T> {
        let res: Option<&T> = if !self.pop_stack.is_empty() {
            Some(&self.pop_stack.back().unwrap().0)
        } else if !self.push_stack.is_empty() {
            Some(&self.push_stack.front().unwrap().0)
        } else {
            None
        };

        res
    }

    pub fn min(&self) -> Option<&T> {
        let res: Option<&T>;

        if self.push_stack.is_empty() || self.pop_stack.is_empty() {
            if self.push_stack.is_empty() && !self.pop_stack.is_empty() {
                res = Some(&self.pop_stack.back().unwrap().1);
            } else if self.pop_stack.is_empty() && !self.push_stack.is_empty() {
                res = Some(&self.push_stack.back().unwrap().1);
            } else {
                res = None
            }
        } else {
            res = Some(cmp::min(
                &self.pop_stack.back().unwrap().1,
                &self.push_stack.back().unwrap().1,
            ))
        }

        res
    }

    pub fn len(&self) -> usize {
        self.push_stack.len() + self.pop_stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.push_stack.is_empty() && self.pop_stack.is_empty()
    }
}
