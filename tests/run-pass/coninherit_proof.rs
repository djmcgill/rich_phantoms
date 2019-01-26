extern crate rich_phantoms;
use rich_phantoms::*;
use std::marker::PhantomData;

fn main() {
    fn s(_: PhantomContravariantInheritSendSync<&'static ()>) {}
    fn f<'a>() {
        let x: PhantomContravariantInheritSendSync<&'a ()> = PhantomData;
        s(x)
    }
}
