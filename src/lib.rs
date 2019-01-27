//! ## Introduction
//! `PhantomData<T>` is [covariant](https://doc.rust-lang.org/nomicon/subtyping.html) and is only `Send+Sync` when `T` is. This may not be the semantics you want - since it is zero-sized it is safe to make send/sync, or you may never want send/sync. If your data type is contravariant then you can't use `PhantomData<T>` either.
//!
//! This library provides phantom types which allow you to specify the variance and send/sync inheritance that you want. See [this discussion](https://github.com/dtolnay/request-for-implementation/issues/21) for more details.
//!
//! Suggestions and contributions welcome!
//!
//! ## Example
//! Here you can see that a non-send/sync inner type doesn\'t affect the wrapper type, and that the `'static` lifetime gets cast into a `'a` lifetime.
//! ```
//! fn main2() {
//!     let x: PhantomCovariantAlwaysSendSync<&'static *const ()> = PhantomData;
//!     fn f<'a>(_: PhantomCovariantAlwaysSendSync<&'a *const ()>) {}
//!     f(x);
//!     let _y: &(dyn Send + Sync) = &x;
//! }
//! ```
//!
//! And here is the opposite example, a send/sync inner type and a contravariant, never-inheriting phantom type - the `'a` is cast to the `'static` lifetime:
//! ```
//! fn main() {
//!     fn s(_: PhantomContravariantNeverSendSync<&'static ()>) {}
//!     fn f<'a>() {
//!         let x: PhantomContravariantNeverSendSync<&'a ()> = PhantomData;
//!         // let _y: &(dyn Send + Sync) = &x; COMPILE ERROR
//!         s(x); // WORKS
//!     }
//! }
//! ```


use core::marker::PhantomData;

/// Is Covariant and always Send/Sync. Construct with `let x = PhantomData`.
pub type PhantomCovariantAlwaysSendSync<T> = PhantomData<PhantomCoFnImpl<T>>;
#[doc(hidden)]
pub struct PhantomCoFnImpl<T: ?Sized>{_a: fn() -> T}

/// Is Covariant and never Send/Sync. Construct with `let x = PhantomData`.
pub type PhantomCovariantNeverSendSync<T> = PhantomData<PhantomCoFnImplNonSync<T>>;
#[doc(hidden)]
pub struct PhantomCoFnImplNonSync<T: ?Sized>{_a: fn() -> T, _b: *const ()}

/// Is Covariant and is Send/Sync if and only if T is. Construct with `let x = PhantomData`.
pub type PhantomCovariantInheritSendSync<T> = PhantomData<PhantomCoFnImplInheritSync<T>>;
#[doc(hidden)]
pub struct PhantomCoFnImplInheritSync<T: ?Sized>{_a: fn() -> T, _b: T}

/// Is Contravariant and always Send/Sync. Construct with `let x = PhantomData`.
pub type PhantomContravariantAlwaysSendSync<T> = PhantomData<PhantomContraFnImpl<T>>;
#[doc(hidden)]
pub struct PhantomContraFnImpl<T: ?Sized>{_a: fn(T)}

/// Is Contravariant and never Send/Sync. Construct with `let x = PhantomData`.
pub type PhantomContravariantNeverSendSync<T> = PhantomData<PhantomContravariantNeverSendSyncImpl<T>>;
#[doc(hidden)]
pub struct PhantomContravariantNeverSendSyncImpl<T: ?Sized>{_a: fn(T), _b: *const ()}

/// Is Contravariant and is Send/Sync if and only if T is. Construct with `let x = PhantomData`.
pub type PhantomContravariantInheritSendSync<T> = PhantomData<PhantomContravariantInheritSendSyncImpl<T>>;
#[doc(hidden)]
pub struct PhantomContravariantInheritSendSyncImpl<T: ?Sized>{_a: fn(T), _b: *const ()}
// These impls are safe because no `PhantomContravariantInheritSendSyncImpl` should ever actually be created.
unsafe impl<T: Send + ?Sized> Send for PhantomContravariantInheritSendSyncImpl<T> {}
unsafe impl<T: Sync + ?Sized> Sync for PhantomContravariantInheritSendSyncImpl<T> {}

/// Is Invariant and always Send/Sync. Construct with `let x = PhantomData`.
pub type PhantomInvariantAlwaysSendSync<T> = PhantomData<PhantomInvariantAlwaysSendSyncImpl<T>>;
#[doc(hidden)]
pub struct PhantomInvariantAlwaysSendSyncImpl<T: ?Sized>{_a: fn(T) -> T}

/// Is Invariant and never Send/Sync. Construct with `let x = PhantomData`.
pub type PhantomInvariantNeverSendSync<T> = PhantomData<PhantomInvariantNeverSendSyncImpl<T>>;
#[doc(hidden)]
pub struct PhantomInvariantNeverSendSyncImpl<T: ?Sized>{_a: fn(T) -> T, _b: *const ()}

/// Is Invariant and is Send/Sync if and only if T is. Construct with `let x = PhantomData`.
pub type PhantomInvariantInheritSendSync<T> = PhantomData<PhantomInvariantInheritSendSyncImpl<T>>;
#[doc(hidden)]
pub struct PhantomInvariantInheritSendSyncImpl<T: ?Sized>{_a: fn(T) -> T, _b: T}
