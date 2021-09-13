#![no_std]
#![feature(decl_macro, type_ascription, core_intrinsics)]

#[cfg(test)]
#[macro_use]
extern crate std;

cfg_if::cfg_if! {
    if #[cfg(feature = "has_alloc")] {
        extern crate alloc;

        #[doc(hidden)]
        pub mod alloc_reexport {
            #[doc(hidden)]
            pub use ::alloc::*;
        }

        #[macro_use]
        pub mod downcast;
    }
}

mod binary_pretty_print;
pub mod bitmap;
mod init_guard;
pub mod math;
#[macro_use]
pub mod pin;

pub use self::{binary_pretty_print::BinaryPrettyPrint, init_guard::InitGuard};

/// This macro should be called at the beginning of functions that create logic errors if they are
/// called more than once. Most commonly this is used for initialization functions.
pub macro assert_first_call
{
    () =>
    {
        assert_first_call!("ASSERTION FAILED: function has already been called");
    },

    ($($arg:tt)+) =>
    {{
        fn assert_first_call()
        {
            use $crate::core_reexport::sync::atomic::{AtomicBool,
                                     ATOMIC_BOOL_INIT,
                                     Ordering};

            static CALLED : AtomicBool = ATOMIC_BOOL_INIT;
            let called = CALLED.swap(true, Ordering::Relaxed);
            assert!(!called, $($arg)+);
        }
        assert_first_call();
    }}
}

/*
 * These are used in macros to prevent weird issues if the using crate doesn't something weird like re-exports
 * another crate as `core` or `alloc`.
 */
#[doc(hidden)]
pub mod core_reexport {
    #[doc(hidden)]
    pub use core::*;
}
