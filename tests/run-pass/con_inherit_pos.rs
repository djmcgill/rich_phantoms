extern crate rich_phantoms;
use rich_phantoms::PhantomContravariantInheritSendSync;
use std::marker::PhantomData;

fn main() {
    let x: PhantomContravariantInheritSendSync<()> = PhantomData;
    let _y: &(dyn Send + Sync) = &x;
}
