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
    let mut v = Vec::new();
    for i in 0..999 {
      v.push(i);
    }
    let mut i = 0;
    b.iter(move || {
        i += 1299827;
        i %= 999;
        black_box(v.binary_search(&i).unwrap());
    })
}
