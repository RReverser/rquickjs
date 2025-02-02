use crate::{qjs, Context, Result, Runtime};
use std::{marker::PhantomData, ptr::NonNull};

/// The internal trait to add JS builting
pub trait Intrinsic {
    /// # Safety
    /// Do not need implement it yourself instead you may use predefined intrinsics from [`intrinsic`] module.
    unsafe fn add_intrinsic(ctx: NonNull<qjs::JSContext>);
}

/// Used for building a [`Context`](struct.Context.html) with a specific set of intrinsics
pub struct ContextBuilder<I>(PhantomData<I>);

macro_rules! intrinsic_impls {
    (@builtin: $($(#[$meta:meta])* $name:ident $func:ident $(($($args:expr),*))*,)*) => {
        $(
            $(#[$meta])*
            pub struct $name;

            impl Intrinsic for $name {
                unsafe fn add_intrinsic(ctx: NonNull<qjs::JSContext>) {
                    qjs::$func(ctx.as_ptr() $(, $($args),*)*);
                }
            }
        )*
    };

    (@tuple: $($($name:ident)*,)*) => {
        $(
            impl<$($name,)*> Intrinsic for ($($name,)*)
            where
                $($name: Intrinsic,)*
            {
                unsafe fn add_intrinsic(_ctx: NonNull<qjs::JSContext>) {
                    $($name::add_intrinsic(_ctx);)*
                }
            }
        )*
    }
}

/// A marker types for intrinsic
///
/// You can select just you need only. If `lto = true` any unused code will be drop by link-time optimizer.
pub mod intrinsic {
    use super::{qjs, Intrinsic, NonNull};

    intrinsic_impls! {
        @builtin:
        /// Add base objects support
        BaseObjects JS_AddIntrinsicBaseObjects,
        /// Add Date object support
        Date JS_AddIntrinsicDate,
        /// Add evaluation support
        Eval JS_AddIntrinsicEval,
        /// Add string normalization
        StringNormalize JS_AddIntrinsicStringNormalize,
        /// Add RegExp compiler
        RegExpCompiler JS_AddIntrinsicRegExpCompiler,
        /// Add RegExp object support
        RegExp JS_AddIntrinsicRegExp,
        /// Add JSON parse and stringify
        Json JS_AddIntrinsicJSON,
        /// Add Proxy object support
        Proxy JS_AddIntrinsicProxy,
        /// Add MapSet object support
        MapSet JS_AddIntrinsicMapSet,
        /// Add Typed Arrays support
        TypedArrays JS_AddIntrinsicTypedArrays,
        /// Add Promise object support
        Promise JS_AddIntrinsicPromise,
        /// Add BigInt support
        BigInt JS_AddIntrinsicBigInt,
        /// Add BigFloat support
        BigFloat JS_AddIntrinsicBigFloat,
        /// Add BigDecimal support
        BigDecimal JS_AddIntrinsicBigDecimal,
        /// Add operator overloading support
        Operators JS_AddIntrinsicOperators,
        /// Enable bignum extension
        BignumExt JS_EnableBignumExt (1),
    }

    /// An alias for [`BaseObjects`]
    pub type Base = BaseObjects;

    /// Add none intrinsics
    pub type None = ();

    /// Add all intrinsics
    pub type All = (
        Base,
        Date,
        Eval,
        StringNormalize,
        RegExpCompiler,
        RegExp,
        Json,
        Proxy,
        MapSet,
        TypedArrays,
        Promise,
        BigInt,
        BigFloat,
        BigDecimal,
        Operators,
        BignumExt,
    );
}

intrinsic_impls! {
    @tuple:
    ,
    A,
    A B,
    A B C,
    A B C D,
    A B C D E,
    A B C D E F,
    A B C D E F G,
    A B C D E F G H,
    A B C D E F G H I,
    A B C D E F G H I J,
    A B C D E F G H I J K,
    A B C D E F G H I J K L,
    A B C D E F G H I J K L M,
    A B C D E F G H I J K L M N,
    A B C D E F G H I J K L M N O,
    A B C D E F G H I J K L M N O P,
    A B C D E F G H I J K L M N O P R,
}

impl Default for ContextBuilder<()> {
    fn default() -> Self {
        ContextBuilder(PhantomData)
    }
}

impl<I: Intrinsic> ContextBuilder<I> {
    pub fn with<J: Intrinsic>(self) -> ContextBuilder<(I, J)> {
        ContextBuilder(PhantomData)
    }

    pub fn build(self, runtime: &Runtime) -> Result<Context> {
        Context::custom::<I>(runtime)
    }
}
