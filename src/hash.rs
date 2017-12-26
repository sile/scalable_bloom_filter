use std::hash::{Hash, Hasher};
use siphasher::sip::SipHasher13;

pub trait NthHash {
    fn nth_hash<T: Hash + ?Sized>(&self, data: &T, nth: usize) -> u64;
}

#[derive(Debug)]
pub struct DefaultHasher;
impl NthHash for DefaultHasher {
    fn nth_hash<T: Hash + ?Sized>(&self, data: &T, nth: usize) -> u64 {
        let mut hasher = SipHasher13::new();
        (nth, data).hash(&mut hasher);
        hasher.finish()
    }
}
