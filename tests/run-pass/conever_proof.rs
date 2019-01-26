extern crate rich_phantoms;
use rich_phantoms::*;
use std::marker::PhantomData;

fn main() {
    let x: PhantomCovariantNeverSendSync<&'static ()> = PhantomData;
    fn f<'a>(_: PhantomCovariantNeverSendSync<&'a ()>) {}
    f(x)
}
