extern crate rich_phantoms;
use rich_phantoms::*;
use std::marker::PhantomData;

fn main() {
    let x: PhantomCovariantInheritSendSync<&'static ()> = PhantomData;
    fn f<'a>(_: PhantomCovariantInheritSendSync<&'a ()>) {}
    f(x)
}
