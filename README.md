
scalable_bloom_filter
======================

[![scalable_bloom_filter](http://meritbadge.herokuapp.com/scalable_bloom_filter)](https://crates.io/crates/scalable_bloom_filter)
[![Documentation](https://docs.rs/scalable_bloom_filter/badge.svg)](https://docs.rs/scalable_bloom_filter)
[![Build Status](https://travis-ci.org/sile/scalable_bloom_filter.svg?branch=master)](https://travis-ci.org/sile/scalable_bloom_filter)
[![Code Coverage](https://codecov.io/gh/sile/scalable_bloom_filter/branch/master/graph/badge.svg)](https://codecov.io/gh/sile/scalable_bloom_filter/branch/master)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A Rust implementation of [Scalable Bloom Filters][sbf].

[sbf]: http://haslab.uminho.pt/cbm/files/dbloom.pdf

TODO
-----

- Reduce the number of hash function invocations:
   > A standard technique from the hashing literature is to use two hash functions `h1(x)` and `h2(x)` to simulate additional hash functions of the form `gi(x) = h1(x)+ih2(x)` ([Less Hashing, Same Performance: Building a Better Bloom Filter
](https://www.eecs.harvard.edu/~michaelm/postscripts/rsa2008.pdf))
