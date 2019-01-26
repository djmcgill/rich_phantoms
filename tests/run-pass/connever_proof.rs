extern crate rich_phantoms;
use rich_phantoms::*;
use std::marker::PhantomData;

fn main() {
    fn s(_: PhantomContravariantNeverSendSync<&'static ()>) {}
    fn f<'a>() {
        let x: PhantomContravariantNeverSendSync<&'a ()> = PhantomData;
        s(x)
    }
}
