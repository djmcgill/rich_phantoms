extern crate rich_phantoms;
use rich_phantoms::PhantomCovariantInheritSendSync;
use std::marker::PhantomData;

fn main() {
    let x: PhantomCovariantInheritSendSync<()> = PhantomData;
    let _y: &(dyn Send + Sync) = &x;
}
