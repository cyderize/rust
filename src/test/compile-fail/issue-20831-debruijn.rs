// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Regression test for #20831: debruijn index account was thrown off
// by the (anonymous) lifetime in `<Self as Publisher>::Output`
// below. Note that changing to a named lifetime made the problem go
// away.

use std::ops::{Shl, Shr};
use std::cell::RefCell;

pub trait Subscriber {
    type Input;
}

pub trait Publisher<'a> {
    type Output;
    fn subscribe(&mut self, Box<Subscriber<Input=Self::Output> + 'a>);
}

pub trait Processor<'a> : Subscriber + Publisher<'a> { }

impl<'a, P> Processor<'a> for P where P : Subscriber + Publisher<'a> { }

struct MyStruct<'a> {
    sub: Box<Subscriber<Input=u64> + 'a>
}

impl<'a> Publisher<'a> for MyStruct<'a> {
    type Output = u64;
    fn subscribe(&mut self, t : Box<Subscriber<Input=<Self as Publisher>::Output> + 'a>) {
        self.subscriber = t;
    }
}

fn main() {}
