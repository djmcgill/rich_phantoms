extern crate rich_phantoms;
use rich_phantoms::PhantomInvariantNeverSendSync;
use std::marker::PhantomData;

fn main() {
    let x: PhantomInvariantNeverSendSync<()> = PhantomData;
    let _y: &(dyn Send + Sync) = &x;
    //~^ ERROR cannot be sent between threads safely
    //~^^ ERROR cannot be shared between threads safely
}
