// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use test::black_box;
use test::Bencher;

enum Cache {
    L1,
    L2,
    L3,
}

fn binary_search(b: &mut Bencher, cache: Cache) {
    let size = match cache {
        Cache::L1 => 1000, // 8kb
        Cache::L2 => 10_000,  // 80kb
        Cache::L3 => 1_000_000,  // 8Mb
    };
    // Only store even numbers.
    let v = (0..size).map(|i| i * 2).collect::<Vec<_>>();
    let mut r = 0usize;
    let max = size * 2;
    b.iter(move || {
        // LCG constants from https://en.wikipedia.org/wiki/Numerical_Recipes.
        r = r.wrapping_mul(1664525).wrapping_add(1013904223);
        // Lookup the whole range to get 50% hits and 50% misses.
        let i = r % max;
        black_box(v.binary_search(&i).is_ok());
    })
}

#[bench]
fn binary_search_hit_l1(b: &mut Bencher) {
    binary_search(b, Cache::L1);
}

#[bench]
fn binary_search_hit_l2(b: &mut Bencher) {
    binary_search(b, Cache::L2);
}

#[bench]
fn binary_search_hit_l3(b: &mut Bencher) {
    binary_search(b, Cache::L3);
}
