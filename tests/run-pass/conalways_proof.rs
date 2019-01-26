extern crate rich_phantoms;
use rich_phantoms::*;
use std::marker::PhantomData;

fn main() {
    fn s(_: PhantomContravariantAlwaysSendSync<&'static ()>) {}
    fn f<'a>() {
        let x: PhantomContravariantAlwaysSendSync<&'a ()> = PhantomData;
        s(x)
    }
}
