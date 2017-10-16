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

#[bench]
fn binary_search(b: &mut Bencher) {
    let v = (0..999).collect::<Vec<_>>();
    let mut i = 0usize;
    b.iter(move || {
        // LCG constants from https://en.wikipedia.org/wiki/Numerical_Recipes.
        i = i.wrapping_mul(1664525).wrapping_add(1013904223);
        black_box(v.binary_search(&(i % 999)).unwrap());
    })
}
