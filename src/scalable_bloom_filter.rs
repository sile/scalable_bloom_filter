use std::hash::Hash;
use bloom_filter::BloomFilter;
use hash::{DefaultHasher, NthHash};

// parameter `s`
const GROWTH_FACTOR: usize = 2;

/// Scalable bloom filter.
///
/// See the [paper] about scalable bloom filters.
///
/// [paper]: http://haslab.uminho.pt/cbm/files/dbloom.pdf
///
/// # Note
///
/// For simplicity, this implementation uses static parameters as follows:
///
/// - growth factor: `s = 2`
/// - tightening ratio: `r = 0.5`
#[derive(Debug)]
pub struct ScalableBloomFilter<T: ?Sized, H = DefaultHasher> {
    hasher: H,
    filters: Vec<BloomFilter<T>>,
}
impl<T: Hash + ?Sized> ScalableBloomFilter<T, DefaultHasher> {
    /// Makes a new `ScalableBloomFilter` instance.
    ///
    /// `initial_capacity` is the expected number of elements in ordinaly cases.
    /// `error_probability` is the maximum allowable probability of false positives.
    ///
    /// # Panics
    ///
    /// `error_probability` must be a value greater than `0.0` and smaller than or equal to `1.0`.
    pub fn new(initial_capacity: usize, error_probability: f64) -> Self {
        Self::with_hasher(initial_capacity, error_probability, DefaultHasher)
    }
}
impl<T: Hash + ?Sized, H: NthHash> ScalableBloomFilter<T, H> {
    /// Makes a new `ScalableBloomFilter` with the given hasher.
    pub fn with_hasher(initial_capacity: usize, error_probability: f64, hasher: H) -> Self {
        assert!(0.0 < error_probability && error_probability <= 1.0);
        let initial_bits =
            error_probability.ln().abs() * (initial_capacity as f64) / 2.0f64.ln().powi(2);
        let slice_count = (1.0 / error_probability).log2().ceil() as usize;
        let filter = BloomFilter::new(initial_bits.ceil() as usize / slice_count, slice_count);
        ScalableBloomFilter {
            filters: vec![filter],
            hasher,
        }
    }

    /// Inserts a element to the filter.
    pub fn insert(&mut self, element: &T) {
        let last = self.filters.len();
        for (i, filter) in self.filters.iter_mut().enumerate() {
            if i + 1 == last {
                filter.insert(element, &self.hasher);
            } else {
                if filter.contains(element, &self.hasher) {
                    return;
                }
            }
        }
        if self.is_growth_needed() {
            self.grow();
        }
    }

    /// Queries whether there is possibility that the element is contains in the filter.
    pub fn contains(&self, element: &T) -> bool {
        self.filters
            .iter()
            .any(|f| f.contains(element, &self.hasher))
    }

    /// The number of bits allocated by the filter.
    ///
    /// As the filter grows, this value will also increase.
    pub fn allocated_bits(&self) -> usize {
        self.filters.iter().map(|f| f.bits().number_of_bits()).sum()
    }

    /// The number of bits used by the filter for storing elements.
    ///
    /// This is the number of one bits in the allocated bits.
    pub fn used_bits(&self) -> usize {
        self.filters
            .iter()
            .map(|f| f.bits().number_of_one_bits())
            .sum()
    }

    fn grow(&mut self) {
        let filter = {
            let last = self.filters.last().expect("Never fails");
            let next_slice_bitwidth = last.slice_bitwidth() * GROWTH_FACTOR;
            let next_number_of_slices = last.number_of_slices() + 1;
            BloomFilter::new(next_slice_bitwidth, next_number_of_slices)
        };
        self.filters.push(filter);
    }

    fn is_growth_needed(&self) -> bool {
        let bits = self.filters.last().expect("Never fails").bits();
        debug_assert_ne!(bits.number_of_one_bits(), 0);
        bits.number_of_bits() / bits.number_of_one_bits() < 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_works() {
        let filter = ScalableBloomFilter::<(), _>::new(18232, 0.001);
        assert_eq!(filter.used_bits(), 0);
        assert_eq!(filter.allocated_bits(), 262144);
    }

    #[test]
    fn insert_and_contains_works() {
        let mut filter = ScalableBloomFilter::new(3, 0.01);
        for x in &["foo", "bar", "baz"] {
            assert!(!filter.contains(x));
            filter.insert(x);
            assert!(filter.contains(x));
        }
    }

    #[test]
    fn growth_works() {
        let mut filter = ScalableBloomFilter::new(32, 0.000001);
        let initial_bits = filter.allocated_bits();

        for i in 0..10000 {
            assert!(!filter.contains(&i));
            filter.insert(&i);
            assert!(filter.contains(&i));
        }
        assert!((0..10000).all(|i| filter.contains(&i)));
        assert_ne!(filter.allocated_bits(), initial_bits);
    }
}
