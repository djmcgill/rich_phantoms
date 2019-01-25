extern crate rich_phantoms;
use rich_phantoms::PhantomInvariantInheritSendSync;
use std::marker::PhantomData;

fn main() {
    let x: PhantomInvariantInheritSendSync<()> = PhantomData;
    let _y: &(dyn Send + Sync) = &x;
}
