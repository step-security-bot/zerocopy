// Copyright 2023 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Documents multiple unsafe blocks with a single safety comment.
///
/// Invoked as:
///
/// ```rust,ignore
/// safety_comment! {
///     // Non-doc comments come first.
///     /// SAFETY:
///     /// Safety comment starts on its own line.
///     macro_1!(args);
///     macro_2! { args };
/// }
/// ```
///
/// The macro invocations are emitted, each decorated with the following
/// attribute: `#[allow(clippy::undocumented_unsafe_blocks)]`.
macro_rules! safety_comment {
    (#[doc = r" SAFETY:"] $(#[doc = $_doc:literal])* $($macro:ident!$args:tt;)*) => {
        #[allow(clippy::undocumented_unsafe_blocks)]
        const _: () = { $($macro!$args;)* };
    }
}

/// Unsafely implements trait(s) for a type.
macro_rules! unsafe_impl {
    // Implement `$trait` for `$ty` with no bounds.
    ($ty:ty: $trait:ty) => {
        unsafe impl $trait for $ty { fn only_derive_is_allowed_to_implement_this_trait() {} }
    };
    // Implement all `$traits` for `$ty` with no bounds.
    ($ty:ty: $($traits:ty),*) => {
        $( unsafe_impl!($ty: $traits); )*
    };
    // For all `$tyvar` with no bounds, implement `$trait` for `$ty`.
    ($tyvar:ident => $trait:ident for $ty:ty) => {
        unsafe impl<$tyvar> $trait for $ty { fn only_derive_is_allowed_to_implement_this_trait() {} }
    };
    // For all `$tyvar: ?Sized` with no bounds, implement `$trait` for `$ty`.
    ($tyvar:ident: ?Sized => $trait:ident for $ty:ty) => {
        unsafe impl<$tyvar: ?Sized> $trait for $ty { fn only_derive_is_allowed_to_implement_this_trait() {} }
    };
    // For all `$tyvar: $bound`, implement `$trait` for `$ty`.
    ($tyvar:ident: $bound:path => $trait:ident for $ty:ty) => {
        unsafe impl<$tyvar: $bound> $trait for $ty { fn only_derive_is_allowed_to_implement_this_trait() {} }
    };
    // For all `$tyvar: $bound + ?Sized`, implement `$trait` for `$ty`.
    ($tyvar:ident: ?Sized + $bound:path => $trait:ident for $ty:ty) => {
        unsafe impl<$tyvar: ?Sized + $bound> $trait for $ty { fn only_derive_is_allowed_to_implement_this_trait() {} }
    };
    // For all `$tyvar: $bound` and for all `const $constvar: $constty`,
    // implement `$trait` for `$ty`.
    ($tyvar:ident: $bound:path, const $constvar:ident: $constty:ty => $trait:ident for $ty:ty) => {
        unsafe impl<$tyvar: $bound, const $constvar: $constty> $trait for $ty {
            fn only_derive_is_allowed_to_implement_this_trait() {}
        }
    };
}

/// Implements trait(s) for a type or verifies the given implementation by
/// referencing an existing (derived) implementation.
///
/// TODO: More detail, including why safety comments aren't required.
macro_rules! impl_or_verify {
    // Implement `$trait` for `$ty` with no bounds.
    ($ty:ty: $trait:ty) => {
        impl_or_verify!(@impl { unsafe_impl!($ty: $trait); });
        impl_or_verify!(@verify $trait, { impl Subtrait for $ty {} });
    };
    // Implement all `$traits` for `$ty` with no bounds.
    ($ty:ty: $($traits:ty),*) => {
        $( foobar!($ty: $traits); )*
    };
    // For all `$tyvar` with no bounds, implement `$trait` for `$ty`.
    ($tyvar:ident => $trait:ident for $ty:ty) => {
        impl_or_verify!(@impl { unsafe_impl!($tyvar => $trait for $ty); });
        impl_or_verify!(@verify $trait, { impl<$tyvar> Subtrait for $ty {} });
    };
    // For all `$tyvar: ?Sized` with no bounds, implement `$trait` for `$ty`.
    ($tyvar:ident: ?Sized => $trait:ident for $ty:ty) => {
        impl_or_verify!(@impl { unsafe_impl!($tyvar: ?Sized => $trait for $ty); });
        impl_or_verify!(@verify $trait, { impl<$tyvar: ?Sized> Subtrait for $ty {} });
    };
    // For all `$tyvar: $bound`, implement `$trait` for `$ty`.
    ($tyvar:ident: $bound:path => $trait:ident for $ty:ty) => {
        impl_or_verify!(@impl { unsafe_impl!($tyvar: $bound => $trait for $ty); });
        impl_or_verify!(@verify $trait, { impl<$tyvar: $bound> Subtrait for $ty {} });
    };
    // For all `$tyvar: $bound + ?Sized`, implement `$trait` for `$ty`.
    ($tyvar:ident: ?Sized + $bound:path => $trait:ident for $ty:ty) => {
        impl_or_verify!(@impl { unsafe_impl!($tyvar: ?Sized + $bound => $trait for $ty); });
        impl_or_verify!(@verify $trait, { impl<$tyvar: ?Sized + $bound> Subtrait for $ty {} });
    };
    // For all `$tyvar: $bound` and for all `const $constvar: $constty`,
    // implement `$trait` for `$ty`.
    ($tyvar:ident: $bound:path, const $constvar:ident: $constty:ty => $trait:ident for $ty:ty) => {
        impl_or_verify!(@impl { unsafe_impl!($tyvar: $bound, const $constvar: $constty => $trait for $ty); });
        impl_or_verify!(@verify $trait, { impl<$tyvar: $bound, const $constvar: $constty> Subtrait for $ty {} });
    };
    (@impl $impl_block:tt) => {
        #[cfg(not(any(feature = "derive", test)))]
        #[allow(clippy::undocumented_unsafe_blocks)]
        const _: () = { $impl_block };
    };
    (@verify $trait:ident, $impl_block:tt) => {
        #[cfg(any(feature = "derive", test))]
        const _: () = {
            trait Subtrait: $trait {}
            $impl_block
        };
    };
}

/// Uses `align_of` to confirm that a type or set of types have alignment 1.
///
/// Note that `align_of<T>` requires `T: Sized`, so this macro doesn't work for
/// unsized types.
macro_rules! assert_unaligned {
    ($ty:ty) => {
        // We only compile this assertion under `cfg(test)` to avoid taking an
        // extra non-dev dependency (and making this crate more expensive to
        // compile for our dependents).
        #[cfg(test)]
        static_assertions::const_assert_eq!(core::mem::align_of::<$ty>(), 1);
    };
    ($($ty:ty),*) => {
        $(assert_unaligned!($ty);)*
    };
}
