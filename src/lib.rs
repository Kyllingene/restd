#![cfg_attr(not(any(feature = "std", test, docsrs)), no_std)]
#![warn(missing_docs)]
//! A re-implementation of various std features. Not perfect nor intended to be.

#[cfg(any(feature = "alloc", test))]
extern crate alloc;

pub mod co;
pub mod fmt;
pub mod io;

// TODO: figure out a way to implement a macro-inclusive glob import properly
// pub mod prelude;

/// The `!` type.
///
/// See the crate [`never-say-never`](https://crates.io/never-say-never)
/// for more info.
pub use never::Never;
mod never {
    #![allow(warnings)]
    pub trait Extract {
        type R;
    }
    impl<T> Extract for fn() -> T {
        type R = T;
    }
    pub type Never = <fn() -> ! as Extract>::R;
}
