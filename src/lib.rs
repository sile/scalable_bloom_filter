//!
//! A Rust implementation of [Scalable Bloom Filters][sbf].
//!
//! [sbf]: http://haslab.uminho.pt/cbm/files/dbloom.pdf
//!
//! # Examples
//!
//! ```
//! use scalable_bloom_filter::ScalableBloomFilter;
//!
//! let mut filter = ScalableBloomFilter::new(1000, 0.001);
//! filter.insert("foo");
//! assert!(filter.contains("foo"));
//! ```
#![warn(missing_docs)]
extern crate siphasher;

pub use scalable_bloom_filter::ScalableBloomFilter;

pub mod hash;

mod bit_vec;
mod bloom_filter;
mod scalable_bloom_filter;
