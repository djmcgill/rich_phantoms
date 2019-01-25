extern crate rich_phantoms;
use rich_phantoms::PhantomInvariantInheritSendSync;
use std::marker::PhantomData;

fn main() {
    let x: PhantomInvariantInheritSendSync<*const ()> = PhantomData;
    let _y: &(dyn Send + Sync) = &x;
    //~^ ERROR cannot be sent between threads safely
    //~^^ ERROR cannot be shared between threads safely
}
