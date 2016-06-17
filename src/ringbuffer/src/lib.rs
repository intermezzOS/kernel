#![no_std]
#![feature(const_fn)]

use core::ptr;

#[derive(Debug)]
pub struct RingBuffer<T> {
    buffer: [Option<T>; 5],
    start: usize,
    end: usize,
}

#[derive(Debug)]
pub enum PushError {
    QueueFull,
}

impl<T> RingBuffer<T> {
    pub const fn new() -> RingBuffer<T> {
        RingBuffer {
            buffer: [None, None, None, None, None],
            start: 0,
            end: 0,
        }
    }

    // buffer: [Some(5), Some(5), Some(5), Some(5), Some(5)
    // start: 4
    // end: 4
    pub fn push(&mut self, val: T) -> Result<(), PushError> {
        let end = self.end;

        if self.buffer.iter().all(Option::is_some) {
            return Err(PushError::QueueFull);
        }

        self.end = (end + 1) % 5;

        self.buffer[end] = Some(val);

        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            let ptr = &self.buffer[self.start] as *const Option<T>;
            let value = ptr::read(ptr);
            self.buffer[self.start] = None;
            self.start = (self.start + 1) % 5;
            value
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn peek(&self) -> Option<T> {
        unsafe {
            let ptr = &self.buffer[self.start] as *const Option<T>;
            ptr::read(ptr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_functionality() {
        let mut buffer = RingBuffer::new();
        buffer.push(5).unwrap();
        let five = buffer.pop();

        assert_eq!(Some(5), five);
    }

    #[test]
    fn pop_actually_removes() {
        let mut buffer = RingBuffer::new();
        buffer.push(5).unwrap();
        buffer.pop();

        assert_eq!(0, buffer.len());
    }

    #[test]
    fn push_actually_adds() {
        let mut buffer = RingBuffer::new();
        buffer.push(5).unwrap();

        match buffer.peek() {
            Some(val) => assert_eq!(5, val),
            None => panic!("got a None where I expected a Some"),
        }
    }

    #[test]
    fn pop_works_with_empty_queue() {
        let mut buffer: RingBuffer<i32> = RingBuffer::new();
        let none = buffer.pop();

        assert!(none.is_none());
    }

    #[test]
    fn intermediate_functionality() {
        let mut buffer = RingBuffer::new();
        buffer.push(5).unwrap();
        buffer.push(10).unwrap();

        assert_eq!(2, buffer.len());

        let five = buffer.pop();
        let ten = buffer.pop();

        assert_eq!(Some(5), five);
        assert_eq!(Some(10), ten);
    }

    #[test]
    fn wraps_around() {
        let mut buffer = RingBuffer::new();

        for i in 0..5 {
            buffer.push(i).unwrap();
            buffer.pop();
        }

        buffer.push(10).unwrap();
        let ten = buffer.pop();

        assert_eq!(Some(10), ten);
    }

    #[test]
    fn push_fails_when_full() {
        let mut buffer = RingBuffer::new();

        for i in 0..5 {
            buffer.push(i).unwrap();
        }

        let result = buffer.push(1000);

        assert!(result.is_err());
    }
}
