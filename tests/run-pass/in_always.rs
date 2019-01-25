extern crate rich_phantoms;
use rich_phantoms::PhantomInvariantAlwaysSendSync;
use std::marker::PhantomData;

fn main() {
    let x: PhantomInvariantAlwaysSendSync<*const ()> = PhantomData;
    let _y: &(dyn Send + Sync) = &x;
}
