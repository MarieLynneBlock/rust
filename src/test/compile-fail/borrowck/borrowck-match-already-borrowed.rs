// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// revisions: ast mir
//[mir]compile-flags: -Z emit-end-regions -Z borrowck-mir

fn main() {
    let mut x = 1;
    let _x = &mut x;
    let _ = match x {
        x => x + 1, //[ast]~ ERROR E0503
                    //[mir]~^ ERROR (Mir) [E0503]
                    //[mir]~| ERROR (Ast) [E0503]
        y => y + 2, //[ast]~ ERROR [E0503]
                    //[mir]~^ ERROR (Mir) [E0503]
                    //[mir]~| ERROR (Ast) [E0503]
    };
}
