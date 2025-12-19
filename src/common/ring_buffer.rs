use std::collections::VecDeque;
use std::sync::RwLock;

#[derive(Debug)]
struct RingBuffer<T> {
    head: RwLock<usize>,
    tail: RwLock<usize>,
    len: usize,
    capacity: usize,
    data: Vec<Option<T>>,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            head: RwLock::new(0),
            tail: RwLock::new(0),
            len: 0,
            capacity,
            data: vec![None; capacity],
        }
    }
}
