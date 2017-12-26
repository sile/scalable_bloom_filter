use std::hash::Hash;
use bloom_filter::BloomFilter;
use hash::{DefaultHasher, NthHash};

#[derive(Debug)]
pub struct ScalableBloomFilter<T: ?Sized, H> {
    filters: Vec<BloomFilter<T, H>>,
}
impl<T: Hash + ?Sized> ScalableBloomFilter<T, DefaultHasher> {
    pub fn new() -> Self {
        ScalableBloomFilter {
            filters: vec![BloomFilter::new(1024, 3)],
        }
    }
}
impl<T: Hash + ?Sized, H: NthHash> ScalableBloomFilter<T, H> {
    pub fn insert(&mut self, value: &T) {
        self.filters.last_mut().expect("Never fails").insert(value);
    }
    pub fn contains(&self, value: &T) -> bool {
        self.filters.iter().any(|f| f.contains(value))
    }
}
