#![forbid(unsafe_code)]

use std::{cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc};
use thiserror::Error;

////////////////////////////////////////////////////////////////////////////////

pub struct Channel<T> {
    buffer: VecDeque<T>,
    closed: bool,
    sender_count: usize,
}

impl<T> Channel<T> {
    pub fn new() -> Channel<T> {
        Self {
            buffer: VecDeque::new(),
            closed: false,
            sender_count: 1,
        }
    }
}

impl<T> Default for Channel<T> {
    fn default() -> Self {
        Self::new()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
#[error("channel is closed")]
pub struct SendError<T> {
    pub value: T,
}

pub struct Sender<T> {
    channel: Rc<RefCell<Channel<T>>>,
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        if self.channel.borrow().closed {
            return Err(SendError { value });
        }

        self.channel.borrow_mut().buffer.push_back(value);
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        (*self.channel).borrow().closed
    }

    pub fn same_channel(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.channel, &other.channel)
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.channel.borrow_mut().sender_count += 1;
        Self {
            channel: self.channel.clone(),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        self.channel.borrow_mut().sender_count -= 1;
        if self.channel.borrow().sender_count == 0 {
            self.channel.borrow_mut().closed = true;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
pub enum ReceiveError {
    #[error("channel is empty")]
    Empty,
    #[error("channel is closed")]
    Closed,
}

pub struct Receiver<T> {
    channel: Rc<RefCell<Channel<T>>>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T, ReceiveError> {
        let val = self.channel.borrow_mut().buffer.pop_front();
        match val {
            None => {
                if self.channel.borrow().closed {
                    return Err(ReceiveError::Closed);
                }
                Err(ReceiveError::Empty)
            }
            Some(elem) => Ok(elem),
        }
    }

    pub fn close(&mut self) {
        self.channel.borrow_mut().closed = true
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.channel.borrow_mut().closed = true;
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let sender = Sender {
        channel: Rc::new(RefCell::new(Channel::new())),
    };

    let receiver = Receiver {
        channel: sender.channel.clone(),
    };

    (sender, receiver)
}
