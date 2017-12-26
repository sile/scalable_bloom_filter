use std::hash::Hash;
use std::marker::PhantomData;

use bit_vec::BitVec;
use hash::NthHash;

#[derive(Debug)]
pub struct BloomFilter<T: ?Sized> {
    bits: BitVec,
    number_of_bits: usize,
    number_of_hashes: usize,
    slice_size: usize,
    _value: PhantomData<T>,
}
impl<T: Hash + ?Sized> BloomFilter<T> {
    pub fn new(number_of_bits: usize, number_of_hashes: usize) -> Self {
        let slice_size = number_of_bits / number_of_hashes + 1;
        BloomFilter {
            bits: BitVec::new(slice_size * number_of_hashes),
            number_of_bits,
            number_of_hashes,
            slice_size,
            _value: PhantomData,
        }
    }
    pub fn insert<H: NthHash>(&mut self, value: &T, hasher: &H) {
        for i in 0..self.number_of_hashes {
            let base = i * self.slice_size;
            let offset = (hasher.nth_hash(value, i) as usize) % self.slice_size;
            self.bits.insert(base + offset);
        }
    }
    pub fn contains<H: NthHash>(&self, value: &T, hasher: &H) -> bool {
        (0..self.number_of_hashes).all(|i| {
            let base = i * self.slice_size;
            let offset = (hasher.nth_hash(value, i) as usize) % self.slice_size;
            self.bits.contains(base + offset)
        })
    }
    pub fn fill_ratio(&self) -> f64 {
        self.bits.fill_ratio()
    }
    pub fn bits(&self) -> usize {
        self.bits.len()
    }
    pub fn slices(&self) -> usize {
        self.slice_size
    }
}
