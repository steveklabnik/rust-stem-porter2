Porter2 English Stemmer
=======================

[![Build Status](https://travis-ci.org/carols10cents/rust-stem-porter2.svg)](https://travis-ci.org/carols10cents/rust-stem-porter2)

This is an INCOMPLETE implementation of the [Porter2 english stemmer](http://snowball.tartarus.org/algorithms/english/stemmer.html) written in Rust. It's a little toy project for me to learn Rust on, while doing something somewhat useful.

I'm currently using rustc 0.12.0-nightly (>72841b128 2014-09-21 20:00:29 +0000) in order to get cargo.

Many thanks to the start that [mrordinaire's porter stemmer in rust](https://github.com/mrordinaire/rust-stem) gave me!!

Compiling
=========

I'm using [Cargo](http://crates.io/)!!! Just run `cargo build`!!!!

Running the tests
=================

I'm using [Cargo](http://crates.io/)!!! Just run `cargo test`!!!!

The tests are really just one test with a lot of cases-- it runs through the words in `test-data/voc.txt` and asserts that the stem of the word matches the corresponding line in `test-data/porter2-output.txt`.

License
=======

MIT. See LICENSE.