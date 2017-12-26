use std::hash::Hash;
use bloom_filter::BloomFilter;
use hash::{DefaultHasher, NthHash};

// parameter `s`
const GROWTH_FACTOR: usize = 2;

// parameter `r`
const FILL_RATIO_LIMIT: f64 = 0.5;

#[derive(Debug)]
pub struct ScalableBloomFilter<T: ?Sized, H = DefaultHasher> {
    hasher: H,
    filters: Vec<BloomFilter<T>>,
}
impl<T: Hash + ?Sized> ScalableBloomFilter<T, DefaultHasher> {
    pub fn new(initial_capacity: usize, error_probability: f64) -> Self {
        Self::with_hasher(initial_capacity, error_probability, DefaultHasher)
    }
}
impl<T: Hash + ?Sized, H: NthHash> ScalableBloomFilter<T, H> {
    pub fn with_hasher(initial_capacity: usize, error_probability: f64, hasher: H) -> Self {
        assert!(0.0 < error_probability && error_probability <= 1.0);
        let initial_bits =
            error_probability.ln().abs() * (initial_capacity as f64) / 2.0f64.ln().powi(2);
        let slice_count = (1.0 / error_probability).log2().ceil() as usize;
        let filter = BloomFilter::new(initial_bits.ceil() as usize, slice_count);
        ScalableBloomFilter {
            filters: vec![filter],
            hasher,
        }
    }
    pub fn insert(&mut self, value: &T) {
        let fill_ratio = {
            let last = self.filters.last_mut().expect("Never fails");
            last.insert(value, &self.hasher);
            last.fill_ratio()
        };
        if fill_ratio >= FILL_RATIO_LIMIT {
            self.grow();
        }
    }
    pub fn contains(&self, value: &T) -> bool {
        self.filters.iter().any(|f| f.contains(value, &self.hasher))
    }

    fn grow(&mut self) {
        let filter = {
            let last = self.filters.last().expect("Never fails");
            let next_bits = last.bits() * GROWTH_FACTOR;
            let next_slices = last.slices() + 1;
            BloomFilter::new(next_bits, next_slices)
        };
        self.filters.push(filter);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut filter = ScalableBloomFilter::new(1000, 0.001);
        filter.insert("foo");
        assert!(filter.contains("foo"));
    }
}
