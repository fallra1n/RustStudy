#![forbid(unsafe_code)]

use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    marker::PhantomData,
    rc::Rc,
};

pub struct LazyCycle<I>
where
    I: Iterator,
    I::Item: Clone,
{
    data: Vec<I::Item>,
    next_id: usize,
    done: bool,
    underlying: I,
}

impl<I> LazyCycle<I>
where
    I: Iterator,
    I::Item: Clone,
{
    fn new(iter: I) -> Self {
        Self {
            data: Vec::new(),
            next_id: 0,
            done: false,
            underlying: iter,
        }
    }

    fn get_after_exhaustion(&mut self) -> Option<I::Item> {
        if self.data.is_empty() {
            return None;
        }

        let len = self.data.len();
        let res = self.data.get(self.next_id % len).cloned();
        self.next_id += 1;
        res
    }

    fn get_next(&mut self) -> Option<I::Item> {
        if self.done {
            return self.get_after_exhaustion();
        }

        match self.underlying.next() {
            None => {
                self.done = true;
                self.get_after_exhaustion()
            }
            Some(val) => {
                let result = Some(val.clone());
                self.data.push(val);
                result
            }
        }
    }
}

impl<I> Iterator for LazyCycle<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_next()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Extract<I: Iterator> {
    data: VecDeque<I::Item>,
    underlying: I,
}

impl<I: Iterator> Extract<I> {
    fn new(mut iter: I, index: usize) -> (Option<I::Item>, Self) {
        let mut after_extracted = VecDeque::new();
        for _ in 0..index {
            match iter.next() {
                None => break,
                Some(val) => after_extracted.push_back(val),
            }
        }

        let extracted = iter.next();

        (
            extracted,
            Self {
                data: after_extracted,
                underlying: iter,
            },
        )
    }
}

impl<I: Iterator> Iterator for Extract<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return self.underlying.next();
        }

        self.data.pop_front()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    obtained: Rc<RefCell<VecDeque<I::Item>>>,
    missed: Rc<RefCell<VecDeque<I::Item>>>,
    done: Rc<Cell<bool>>,
    underlying: Rc<RefCell<I>>,
}

impl<I> Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    fn new(iter: I) -> (Self, Self) {
        let first = Self {
            obtained: Rc::new(RefCell::new(VecDeque::new())),
            missed: Rc::new(RefCell::new(VecDeque::new())),
            done: Rc::new(Cell::new(false)),
            underlying: Rc::new(RefCell::new(iter)),
        };

        let second = Self {
            obtained: first.missed.clone(),
            missed: first.obtained.clone(),
            done: first.done.clone(),
            underlying: first.underlying.clone(),
        };

        (first, second)
    }
}

impl<I> Iterator for Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.missed.borrow_mut().is_empty() {
            return self.missed.borrow_mut().pop_front();
        }

        if self.done.get() {
            return None;
        }

        return match self.underlying.borrow_mut().next() {
            None => {
                self.done.set(true);
                None
            }
            Some(next_val) => {
                self.obtained.borrow_mut().push_back(next_val.clone());
                Some(next_val)
            }
        };
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    data: VecDeque<(V, Vec<I::Item>)>,
    marker: PhantomData<F>,
}

impl<I, F, V> GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    fn new(iter: I, mut func: F) -> Self {
        let mut data: VecDeque<(V, Vec<I::Item>)> = VecDeque::new();
        for val in iter {
            let key = func(&val);

            match data.back_mut() {
                None => data.push_back((key, vec![val])),
                Some((k, vals)) => {
                    if k == &key {
                        vals.push(val);
                    } else {
                        data.push_back((key, vec![val]));
                    }
                }
            }
        }

        Self {
            data,
            marker: PhantomData,
        }
    }
}

impl<I, F, V> Iterator for GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    type Item = (V, Vec<I::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop_front()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait ExtendedIterator: Iterator {
    fn lazy_cycle(self) -> LazyCycle<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        LazyCycle::new(self)
    }

    fn extract(self, index: usize) -> (Option<Self::Item>, Extract<Self>)
    where
        Self: Sized,
    {
        Extract::new(self, index)
    }

    fn tee(self) -> (Tee<Self>, Tee<Self>)
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Tee::new(self)
    }

    fn group_by<F, V>(self, func: F) -> GroupBy<Self, F, V>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> V,
        V: Eq,
    {
        GroupBy::new(self, func)
    }
}

impl<I: Iterator> ExtendedIterator for I {}
