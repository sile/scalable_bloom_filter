use std::hash::Hash;
use std::marker::PhantomData;

use bit_vec::BitVec;
use hash::NthHash;

#[derive(Debug)]
pub struct BloomFilter<T: ?Sized> {
    bits: BitVec,
    slice_bitwidth: usize,
    number_of_slices: usize,
    _value: PhantomData<T>,
}
impl<T: Hash + ?Sized> BloomFilter<T> {
    pub fn new(slice_bitwidth: usize, number_of_slices: usize) -> Self {
        BloomFilter {
            bits: BitVec::new(slice_bitwidth * number_of_slices),
            slice_bitwidth,
            number_of_slices,
            _value: PhantomData,
        }
    }

    #[inline]
    pub fn insert<H: NthHash>(&mut self, value: &T, hasher: &H) {
        for i in 0..self.number_of_slices {
            let base = i * self.slice_bitwidth;
            let offset = (hasher.nth_hash(value, i) as usize) % self.slice_bitwidth;
            self.bits.insert(base + offset);
        }
    }

    #[inline]
    pub fn contains<H: NthHash>(&self, value: &T, hasher: &H) -> bool {
        (0..self.number_of_slices).all(|i| {
            let base = i * self.slice_bitwidth;
            let offset = (hasher.nth_hash(value, i) as usize) % self.slice_bitwidth;
            self.bits.contains(base + offset)
        })
    }

    pub fn bits(&self) -> &BitVec {
        &self.bits
    }

    pub fn slice_bitwidth(&self) -> usize {
        self.slice_bitwidth
    }

    pub fn number_of_slices(&self) -> usize {
        self.number_of_slices
    }
}

#[cfg(test)]
mod test {
    use hash::DefaultHasher;
    use super::*;

    #[test]
    fn new_works() {
        let filter = BloomFilter::<()>::new(128, 4);
        assert_eq!(filter.bits().number_of_bits(), 128 * 4);
        assert_eq!(filter.slice_bitwidth(), 128);
        assert_eq!(filter.number_of_slices(), 4);
    }

    #[test]
    fn insert_and_contains_works() {
        let mut filter = BloomFilter::new(128, 4);
        let hasher = &DefaultHasher;

        for (i, x) in ["foo", "bar", "baz"].iter().enumerate() {
            assert!(!filter.contains(x, hasher));
            filter.insert(x, hasher);
            assert!(filter.contains(x, hasher));
            assert_eq!(filter.bits().number_of_one_bits(), (i + 1) * 4);
        }
    }

    #[test]
    fn dense_filter_works() {
        let mut filter = BloomFilter::new(128, 4);
        let hasher = &DefaultHasher;
        for i in 0..127 {
            filter.insert(&i, hasher);
            assert!(filter.bits().number_of_one_bits() <= (i + 1) * 4);
        }
        assert!((0..127).all(|i| filter.contains(&i, hasher)));
        assert_ne!(
            filter.bits().number_of_one_bits(),
            filter.bits().number_of_bits()
        );

        // Fills all bits
        for i in 0..1000 {
            filter.insert(&i, hasher);
        }
        assert!(filter.contains(&1001, hasher)); // false positive
        assert_eq!(
            filter.bits().number_of_one_bits(),
            filter.bits().number_of_bits()
        );
    }
}
