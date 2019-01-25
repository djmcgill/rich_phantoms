// It should support any combination of {Covariant, Contravariant, Invariant} × {AlwaysSendSync, NeverSendSync, InheritSendSync}.

use core::marker::PhantomData;

// Is Covariant and Send/Sync
// ```
//    use rich_phantoms::*;
//    use core::marker::PhantomData;
//    let x: PhantomCovariantAlwaysSendSync<&'static str> = PhantomData;
//    let _y: &(dyn Send + Sync) = &x;
//    fn f<'a>(_: PhantomCovariantAlwaysSendSync<&'a str>) {}
//    f(x);
// ```
pub type PhantomCovariantAlwaysSendSync<T> = PhantomData<PhantomCoFnWorkaround<T>>;
pub struct PhantomCoFnWorkaround<T: ?Sized>(fn() -> T);

// Is Covariant:
// ```
//    use rich_phantoms::*;
//    use core::marker::PhantomData;
//    let x: PhantomCovariantNeverSendSync<&'static str> = PhantomData;
//    fn f<'a>(_: PhantomCovariantNeverSendSync<&'a str>) {}
//    f(x);
// ```
// Is not Send/Sync
// ```compile_fail
//    use rich_phantoms::*;
//    use core::marker::PhantomData;
//    let x: PhantomCovariantNeverSendSync<&'static str> = PhantomData;
//    let _y: &(dyn Send + Sync) = &x;
// ```
pub type PhantomCovariantNeverSendSync<T> = PhantomData<PhantomCoFnWorkaroundNonSync<T>>;
pub struct PhantomCoFnWorkaroundNonSync<T: ?Sized>(fn() -> T, *const ());

// Is Covariant:
// ```
//    use rich_phantoms::*;
//    use core::marker::PhantomData;
//    let x: PhantomCovariantInheritSendSync<&'static str> = PhantomData;
//    fn f<'a>(_: PhantomCovariantInheritSendSync<&'a str>) {}
//    f(x);
// ```
// Is Send/Sync when T is
// ```
//    use rich_phantoms::*;
//    use core::marker::PhantomData;
//    let x: PhantomCovariantInheritSendSync<&'static str> = PhantomData;
//    let _y: &(dyn Send + Sync) = &x;
// ```
// Is not Send/Sync when T is not
// ```compile_fail
//    use rich_phantoms::*;
//    use core::marker::PhantomData;
//    let x: PhantomCovariantInheritSendSync<&'static *mut ()> = PhantomData;
//    let _y: &(dyn Send + Sync) = &x;
// ```
pub type PhantomCovariantInheritSendSync<T> = PhantomData<PhantomCoFnWorkaroundInheritSync<T>>;
pub struct PhantomCoFnWorkaroundInheritSync<T: ?Sized>(fn() -> T, T);

// Is Contravariant and Send/Sync:
// use rich_phantoms::*;
// use core::marker::PhantomData;
// fn outer<'a>(_: &'a str) {
//     let x: PhantomContravariantAlwaysSendSync<&'a str> = PhantomData;
//
//     let _y: &(dyn Send + Sync) = &x;
//
//     fn f(_: PhantomContravariantAlwaysSendSync<&'static str>) {}
//     f(x);
// }
// ```
pub type PhantomContravariantAlwaysSendSync<T> = PhantomData<PhantomContraFnWorkaround<T>>;
pub struct PhantomContraFnWorkaround<T: ?Sized>(fn(T));

// Is Contravariant:
// ```
// use rich_phantoms::*;
// use core::marker::PhantomData;
// fn outer<'a>(_: &'a str) {
//     let x: PhantomContravariantNeverSendSync<&'a str> = PhantomData;
//     fn f(_: PhantomContravariantNeverSendSync<&'static str>) {}
//     f(x);
// }
// ```
pub type PhantomContravariantNeverSendSync<T> = PhantomData<PhantomContravariantNeverSendSyncWorkaround<T>>;
pub struct PhantomContravariantNeverSendSyncWorkaround<T>(fn(T), *const ());

// Is Contravariant:
// ```
// use rich_phantoms::*;
// use core::marker::PhantomData;
// fn outer<'a>(_: &'a str) {
//     let x: PhantomContravariantInheritSendSync<&'a str> = PhantomData;
//     fn f(_: PhantomContravariantInheritSendSync<&'static str>) {}
//     f(x);
// }
// ```
pub type PhantomContravariantInheritSendSync<T> = PhantomData<PhantomContravariantInheritSendSyncWorkaround<T>>;
pub struct PhantomContravariantInheritSendSyncWorkaround<T>(fn(T), T);

pub type PhantomInvariantAlwaysSendSync<T> = PhantomData<PhantomInvariantAlwaysSendSyncWorkaround<T>>;
pub struct PhantomInvariantAlwaysSendSyncWorkaround<T>(fn(T) -> T);

pub type PhantomInvariantNeverSendSync<T> = PhantomData<PhantomInvariantNeverSendSyncWorkaround<T>>;
pub struct PhantomInvariantNeverSendSyncWorkaround<T>(fn(T) -> T, *const ());

pub type PhantomInvariantInheritSendSync<T> = PhantomData<PhantomInvariantInheritSendSyncWorkaround<T>>;
pub struct PhantomInvariantInheritSendSyncWorkaround<T>(fn(T) -> T, T);
