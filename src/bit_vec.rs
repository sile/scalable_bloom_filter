use std::mem;

#[derive(Debug)]
pub struct BitVec {
    bits: Vec<usize>,
}
impl BitVec {
    pub fn new(capacity: usize) -> Self {
        BitVec {
            bits: vec![0; capacity / mem::size_of::<usize>() + 1],
        }
    }
    pub fn insert(&mut self, index: usize) {
        let base = index / mem::size_of::<usize>();
        let offset = index % mem::size_of::<usize>();
        *self.bits.get_mut(base).expect("TODO") |= 1 << offset;
    }
    pub fn contains(&self, index: usize) -> bool {
        let base = index / mem::size_of::<usize>();
        let offset = index % mem::size_of::<usize>();
        (self.bits[base] & (1 << offset)) != 0
    }
}
