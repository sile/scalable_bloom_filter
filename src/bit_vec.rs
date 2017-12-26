use std::mem;

#[derive(Debug)]
pub struct BitVec {
    bits: Vec<usize>,
    one_bits: usize,
}
impl BitVec {
    pub fn new(capacity: usize) -> Self {
        BitVec {
            bits: vec![0; capacity / mem::size_of::<usize>() + 1],
            one_bits: 0,
        }
    }
    pub fn insert(&mut self, index: usize) {
        let base = index / mem::size_of::<usize>();
        let offset = index % mem::size_of::<usize>();
        if (self.bits[base] & (1 << offset)) == 0 {
            *self.bits.get_mut(base).expect("TODO") |= 1 << offset;
            self.one_bits += 1;
        }
    }
    pub fn contains(&self, index: usize) -> bool {
        let base = index / mem::size_of::<usize>();
        let offset = index % mem::size_of::<usize>();
        (self.bits[base] & (1 << offset)) != 0
    }
    pub fn fill_ratio(&self) -> f64 {
        (self.one_bits as f64) / (self.bits.len() * 8) as f64
    }
    pub fn len(&self) -> usize {
        self.bits.len() * 8
    }
}
