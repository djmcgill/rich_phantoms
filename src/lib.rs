use core::marker::PhantomData;

/// Is Covariant and always Send/Sync. Construct with `let x = PhantomData`.
pub type PhantomCovariantAlwaysSendSync<T> = PhantomData<PhantomCoFnWorkaround<T>>;
#[doc(hidden)]
pub struct PhantomCoFnWorkaround<T: ?Sized>{_a: fn() -> T}

/// Is Covariant and never Send/Sync. Construct with `let x = PhantomData`.
pub type PhantomCovariantNeverSendSync<T> = PhantomData<PhantomCoFnWorkaroundNonSync<T>>;
#[doc(hidden)]
pub struct PhantomCoFnWorkaroundNonSync<T: ?Sized>{_a: fn() -> T, _b: *const ()}

/// Is Covariant and is Send/Sync if and only if T is. Construct with `let x = PhantomData`.
pub type PhantomCovariantInheritSendSync<T> = PhantomData<PhantomCoFnWorkaroundInheritSync<T>>;
#[doc(hidden)]
pub struct PhantomCoFnWorkaroundInheritSync<T: ?Sized>{_a: fn() -> T, _b: T}

/// Is Contravariant and always Send/Sync. Construct with `let x = PhantomData`.
pub type PhantomContravariantAlwaysSendSync<T> = PhantomData<PhantomContraFnWorkaround<T>>;
#[doc(hidden)]
pub struct PhantomContraFnWorkaround<T: ?Sized>{_a: fn(T)}

/// Is Contravariant and never Send/Sync. Construct with `let x = PhantomData`.
pub type PhantomContravariantNeverSendSync<T> = PhantomData<PhantomContravariantNeverSendSyncWorkaround<T>>;
#[doc(hidden)]
pub struct PhantomContravariantNeverSendSyncWorkaround<T: ?Sized>{_a: fn(T), _b: *const ()}

/// Is Contravariant and is Send/Sync if and only if T is. Construct with `let x = PhantomData`.
pub type PhantomContravariantInheritSendSync<T> = PhantomData<PhantomContravariantInheritSendSyncWorkaround<T>>;
#[doc(hidden)]
pub struct PhantomContravariantInheritSendSyncWorkaround<T: ?Sized>{_a: fn(T), _b: *const ()}
// These impls are safe because no `PhantomContravariantInheritSendSyncWorkaround` should ever actually be created.
unsafe impl<T: Send + ?Sized> Send for PhantomContravariantInheritSendSyncWorkaround<T> {}
unsafe impl<T: Sync + ?Sized> Sync for PhantomContravariantInheritSendSyncWorkaround<T> {}

/// Is Invariant and always Send/Sync. Construct with `let x = PhantomData`.
pub type PhantomInvariantAlwaysSendSync<T> = PhantomData<PhantomInvariantAlwaysSendSyncWorkaround<T>>;
#[doc(hidden)]
pub struct PhantomInvariantAlwaysSendSyncWorkaround<T: ?Sized>{_a: fn(T) -> T}

/// Is Invariant and never Send/Sync. Construct with `let x = PhantomData`.
pub type PhantomInvariantNeverSendSync<T> = PhantomData<PhantomInvariantNeverSendSyncWorkaround<T>>;
#[doc(hidden)]
pub struct PhantomInvariantNeverSendSyncWorkaround<T: ?Sized>{_a: fn(T) -> T, _b: *const ()}

/// Is Invariant and is Send/Sync if and only if T is. Construct with `let x = PhantomData`.
pub type PhantomInvariantInheritSendSync<T> = PhantomData<PhantomInvariantInheritSendSyncWorkaround<T>>;
#[doc(hidden)]
pub struct PhantomInvariantInheritSendSyncWorkaround<T: ?Sized>{_a: fn(T) -> T, _b: T}
