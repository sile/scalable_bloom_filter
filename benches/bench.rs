#![feature(test)]
extern crate scalable_bloom_filter;
extern crate test;

use scalable_bloom_filter::ScalableBloomFilter;
use test::Bencher;

#[bench]
fn insert_n1000_p01(b: &mut Bencher) {
    let mut filter = ScalableBloomFilter::new(1000, 0.1);
    let mut i = 0;
    b.iter(|| {
        filter.insert(&i);
        i += 1;
    });
}

#[bench]
fn insert_n1000_p001(b: &mut Bencher) {
    let mut filter = ScalableBloomFilter::new(1000, 0.01);
    let mut i = 0;
    b.iter(|| {
        filter.insert(&i);
        i += 1;
    });
}

#[bench]
fn insert_n1000_p0001(b: &mut Bencher) {
    let mut filter = ScalableBloomFilter::new(1000, 0.001);
    let mut i = 0;
    b.iter(|| {
        filter.insert(&i);
        i += 1;
    });
}

#[bench]
fn insert_n10000_p01(b: &mut Bencher) {
    let mut filter = ScalableBloomFilter::new(10_000, 0.1);
    let mut i = 0;
    b.iter(|| {
        filter.insert(&i);
        i += 1;
    });
}

#[bench]
fn insert_n10000_p001(b: &mut Bencher) {
    let mut filter = ScalableBloomFilter::new(10_000, 0.01);
    let mut i = 0;
    b.iter(|| {
        filter.insert(&i);
        i += 1;
    });
}

#[bench]
fn insert_n10000_p0001(b: &mut Bencher) {
    let mut filter = ScalableBloomFilter::new(10_000, 0.001);
    let mut i = 0;
    b.iter(|| {
        filter.insert(&i);
        i += 1;
    });
}

#[bench]
fn insert_n100000_p01(b: &mut Bencher) {
    let mut filter = ScalableBloomFilter::new(100_000, 0.1);
    let mut i = 0;
    b.iter(|| {
        filter.insert(&i);
        i += 1;
    });
}

#[bench]
fn insert_n100000_p001(b: &mut Bencher) {
    let mut filter = ScalableBloomFilter::new(100_000, 0.01);
    let mut i = 0;
    b.iter(|| {
        filter.insert(&i);
        i += 1;
    });
}

#[bench]
fn insert_n100000_p0001(b: &mut Bencher) {
    let mut filter = ScalableBloomFilter::new(100_000, 0.001);
    let mut i = 0;
    b.iter(|| {
        filter.insert(&i);
        i += 1;
    });
}
