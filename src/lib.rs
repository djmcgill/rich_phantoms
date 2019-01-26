use core::marker::PhantomData;

/// Is Covariant and always Send/Sync
pub type PhantomCovariantAlwaysSendSync<T> = PhantomData<PhantomCoFnWorkaround<T>>;
pub struct PhantomCoFnWorkaround<T: ?Sized>{_a: fn() -> T}

/// Is Covariant and never Send/Sync
pub type PhantomCovariantNeverSendSync<T> = PhantomData<PhantomCoFnWorkaroundNonSync<T>>;
pub struct PhantomCoFnWorkaroundNonSync<T: ?Sized>{_a: fn() -> T, _b: *const ()}

/// Is Covariant and is Send/Sync if and only if T is
pub type PhantomCovariantInheritSendSync<T> = PhantomData<PhantomCoFnWorkaroundInheritSync<T>>;
pub struct PhantomCoFnWorkaroundInheritSync<T: ?Sized>{_a: fn() -> T, _b: T}

/// Is Contravariant and always Send/Sync
pub type PhantomContravariantAlwaysSendSync<T> = PhantomData<PhantomContraFnWorkaround<T>>;
pub struct PhantomContraFnWorkaround<T: ?Sized>{_a: fn(T)}

/// Is Contravariant and never Send/Sync
pub type PhantomContravariantNeverSendSync<T> = PhantomData<PhantomContravariantNeverSendSyncWorkaround<T>>;
pub struct PhantomContravariantNeverSendSyncWorkaround<T>{_a: fn(T), _b: *const ()}

/// Is Contravariant and is Send/Sync if and only if T is
pub type PhantomContravariantInheritSendSync<T> = PhantomData<PhantomContravariantInheritSendSyncWorkaround<T>>;
pub struct PhantomContravariantInheritSendSyncWorkaround<T>{_a: fn(T), _b: *const ()}
unsafe impl<T: Send> Send for PhantomContravariantInheritSendSyncWorkaround<T> {}
unsafe impl<T: Sync> Sync for PhantomContravariantInheritSendSyncWorkaround<T> {}

/// Is Invariant and always Send/Sync
pub type PhantomInvariantAlwaysSendSync<T> = PhantomData<PhantomInvariantAlwaysSendSyncWorkaround<T>>;
pub struct PhantomInvariantAlwaysSendSyncWorkaround<T>{_a: fn(T) -> T}

/// Is Invariant and never Send/Sync
pub type PhantomInvariantNeverSendSync<T> = PhantomData<PhantomInvariantNeverSendSyncWorkaround<T>>;
pub struct PhantomInvariantNeverSendSyncWorkaround<T>{_a: fn(T) -> T, _b: *const ()}

/// Is Invariant and is Send/Sync if and only if T is
pub type PhantomInvariantInheritSendSync<T> = PhantomData<PhantomInvariantInheritSendSyncWorkaround<T>>;
pub struct PhantomInvariantInheritSendSyncWorkaround<T>{_a: fn(T) -> T, _b: T}
