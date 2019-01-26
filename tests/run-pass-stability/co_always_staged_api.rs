#![feature(staged_api)]
#![feature(const_fn)]

extern crate rich_phantoms;
use rich_phantoms::PhantomCovariantAlwaysSendSync;
use std::marker::PhantomData;
const fn main() {
    let _x: PhantomCovariantAlwaysSendSync<()> = PhantomData;
}
