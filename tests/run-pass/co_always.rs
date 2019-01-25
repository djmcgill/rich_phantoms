extern crate rich_phantoms;
use rich_phantoms::PhantomCovariantAlwaysSendSync;
use std::marker::PhantomData;

fn main() {
    let x: PhantomCovariantAlwaysSendSync<*const ()> = PhantomData;
    let _y: &(dyn Send + Sync) = &x;
}
