# Rich Phantoms
An implementation of phantom types in rust with control over variance and inheritance of send/sync.

They are all just type aliases for `PhantomData`, so work in `const` and non-sized contexts too.

## Introduction
`PhantomData<T>` is [covariant](https://doc.rust-lang.org/nomicon/subtyping.html) and is only `Send+Sync` when `T` is. This may not be the semantics you want - since it is zero-sized it is safe to make send/sync, or you may never want send/sync. If your data type is contravariant then you can't use `PhantomData<T>` either.

This library provides phantom types which allow you to specify the variance and send/sync inheritance that you want. See [this discussion](https://github.com/dtolnay/request-for-implementation/issues/21) for more details.

Suggestions and contributions welcome!

## Example
Here you can see that a non-send/sync inner type doesn\'t affect the wrapper type, and that the `'const` lifetime gets cast into a `'a` lifetime.
```
fn main() {
    let x: PhantomCovariantAlwaysSendSync<*const ()> = PhantomData;
    fn f<'a>(_: PhantomCovariantAlwaysSendSync<&'a ()>) {}
    f(x);
    let _y: &(dyn Send + Sync) = &x;
}
```

And here is the opposite example, a send/sync inner type and a contravariant, never-inheriting phantom type - the `'a` is cast to the `'static` lifetime:
```
fn main() {
    fn s(_: PhantomContravariantNeverSendSync<&'static ()>) {}
    fn f<'a>() {
        let x: PhantomContravariantAlwaysSendSync<&'a ()> = PhantomData;
        // let _y: &(dyn Send + Sync) = &x; COMPILE ERROR
        s(x); // WORKS
    }
}
```
## Tests
There are tests for all the variants which prove the send/sync inheritance (including compile-fail tests when the phantom type should not be send/sync), as well as (positive) tests for the correct kind of variance.

Additionally there is a feature-gated test which runs under `-Z force-unstable-if-unmarked`, `#![feature(staged_api)]`, and  `#![feature(const_fn)]`. To run this test use (on nightly): `cargo test --features unstable-test`.
