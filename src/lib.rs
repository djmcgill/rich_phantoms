// It should support any combination of {Covariant, Contravariant, Invariant} × {AlwaysSendSync, NeverSendSync, InheritSendSync}.

use core::marker::PhantomData;

/// Is Covariant and Send/Sync
/// ```
///    use rich_phantoms::*;
///    use core::marker::PhantomData;
///    let _x: PhantomCovariantAlwaysSendSync<&'static str> = PhantomData;
///    let _y: &(dyn Send + Sync) = &_x;
///    fn f<'a>(_: PhantomCovariantAlwaysSendSync<&'a str>) {}
///    f(_x);
/// ```
pub type PhantomCovariantAlwaysSendSync<T> = PhantomData<PhantomCoFnWorkaround<T>>;
pub struct PhantomCoFnWorkaround<T: ?Sized>(fn() -> T);

/// Is Covariant:
/// ```
///    use rich_phantoms::*;
///    use core::marker::PhantomData;
///    let _x: PhantomCovariantNeverSendSync<&'static str> = PhantomData;
///    fn f<'a>(_: PhantomCovariantNeverSendSync<&'a str>) {}
///    f(_x);
/// ```
/// Is not Send/Sync
/// ```compile_fail
///    use rich_phantoms::*;
///    use core::marker::PhantomData;
///    let _x: PhantomCovariantNeverSendSync<&'static str> = PhantomData;
///    let _y: &(dyn Send + Sync) = &_x;
/// ```
pub type PhantomCovariantNeverSendSync<T> = PhantomData<PhantomCoFnWorkaroundNonSync<T>>;
pub struct PhantomCoFnWorkaroundNonSync<T: ?Sized>(fn() -> T, *mut ());

/// Is Covariant:
/// ```
///    use rich_phantoms::*;
///    use core::marker::PhantomData;
///    let _x: PhantomCovariantInheritSendSync<&'static str> = PhantomData;
///    fn f<'a>(_: PhantomCovariantInheritSendSync<&'a str>) {}
///    f(_x);
/// ```
/// Is Send/Sync when T is
/// ```
///    use rich_phantoms::*;
///    use core::marker::PhantomData;
///    let _x: PhantomCovariantInheritSendSync<&'static str> = PhantomData;
///    let _y: &(dyn Send + Sync) = &_x;
/// ```
/// Is not Send/Sync when T is not
/// ```compile_fail
///    use rich_phantoms::*;
///    use core::marker::PhantomData;
///    let _x: PhantomCovariantInheritSendSync<&'static *mut ()> = PhantomData;
///    let _y: &(dyn Send + Sync) = &_x;
/// ```
pub type PhantomCovariantInheritSendSync<T> = PhantomData<PhantomCoFnWorkaroundInheritSync<T>>;
pub struct PhantomCoFnWorkaroundInheritSync<T: ?Sized>(fn() -> T, T);

/// Is Contravariant and Send/Sync:
/// ```
/// use rich_phantoms::*;
/// use core::marker::PhantomData;
/// fn outer<'a>(_: &'a str) {
///     let _x: PhantomContravariantAlwaysSendSync<&'a str> = PhantomData;
///
///     let _y: &(dyn Send + Sync) = &_x;
///
///     fn f(_: PhantomContravariantAlwaysSendSync<&'static str>) {}
///     f(_x);
/// }
/// ```
pub type PhantomContravariantAlwaysSendSync<T> = PhantomData<PhantomContraFnWorkaround<T>>;
pub struct PhantomContraFnWorkaround<T: ?Sized>(fn(T));


// TODO:
// PhantomContravariantNeverSendSync
// PhantomContravariantInheritSendSync
// PhantomInvariantAlwaysSendSync
// PhantomInvariantNeverSendSync
// PhantomInvariantInheritSendSync
