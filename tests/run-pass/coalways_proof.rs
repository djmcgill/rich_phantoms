extern crate rich_phantoms;
use rich_phantoms::*;
use std::marker::PhantomData;

fn main() {
    let x: PhantomCovariantAlwaysSendSync<&'static ()> = PhantomData;
    fn f<'a>(_: PhantomCovariantAlwaysSendSync<&'a ()>) {}
    f(x)
}
