extern crate rich_phantoms;
use rich_phantoms::PhantomContravariantAlwaysSendSync;
use std::marker::PhantomData;

fn main() {
    let x: PhantomContravariantAlwaysSendSync<*const ()> = PhantomData;
    let _y: &(dyn Send + Sync) = &x;
}
