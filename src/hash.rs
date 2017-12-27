//! `NthHash` trait and its default implementation.
use std::hash::{Hash, Hasher};
use siphasher::sip::SipHasher13;

/// Hash function for bloom filters.
pub trait NthHash {
    /// Calculates the `nth` hash value of `data`.
    fn nth_hash<T: Hash + ?Sized>(&self, data: &T, nth: usize) -> u64;
}

/// The default implementation of `NthHash` trait.
#[derive(Debug)]
pub struct DefaultHasher;
impl NthHash for DefaultHasher {
    fn nth_hash<T: Hash + ?Sized>(&self, data: &T, nth: usize) -> u64 {
        let mut hasher = SipHasher13::new();
        (nth, data).hash(&mut hasher);
        hasher.finish()
    }
}
