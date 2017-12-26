use std::hash::Hash;
use std::marker::PhantomData;

use bit_vec::BitVec;
use hash::{DefaultHasher, NthHash};

#[derive(Debug)]
pub struct BloomFilter<T: ?Sized, H = DefaultHasher> {
    bits: BitVec,
    hasher: H,
    number_of_bits: usize,
    number_of_hashes: usize,
    slice_size: usize,
    _value: PhantomData<T>,
}
impl<T: Hash + ?Sized> BloomFilter<T, DefaultHasher> {
    pub fn new(number_of_bits: usize, number_of_hashes: usize) -> Self {
        Self::with_hasher(number_of_bits, number_of_hashes, DefaultHasher)
    }
}
impl<T: Hash + ?Sized, H: NthHash> BloomFilter<T, H> {
    pub fn with_hasher(number_of_bits: usize, number_of_hashes: usize, hasher: H) -> Self {
        let slice_size = number_of_bits / number_of_hashes + 1;
        BloomFilter {
            bits: BitVec::new(slice_size * number_of_hashes),
            hasher,
            number_of_bits,
            number_of_hashes,
            slice_size,
            _value: PhantomData,
        }
    }
    pub fn insert(&mut self, value: &T) {
        for i in 0..self.number_of_hashes {
            let base = i * self.slice_size;
            let offset = (self.hasher.nth_hash(value, i) as usize) % self.slice_size;
            self.bits.insert(base + offset);
        }
    }
    pub fn contains(&self, value: &T) -> bool {
        (0..self.number_of_hashes).all(|i| {
            let base = i * self.slice_size;
            let offset = (self.hasher.nth_hash(value, i) as usize) % self.slice_size;
            self.bits.contains(base + offset)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut filter = BloomFilter::new(100, 3);
        filter.insert("foo");
        assert!(filter.contains("foo"));
    }
}
