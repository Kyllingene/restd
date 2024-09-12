#![cfg_attr(not(any(feature = "std", test, docsrs)), no_std)]

pub mod co;
pub mod fmt;
pub mod io;
pub mod prelude;

/// The `!` type.
///
/// See the crate [`never-say-never`](https://crates.io/never-say-never)
/// for more info.
pub use never::Never;
mod never {
    pub trait Extract {
        type R;
    }
    impl<T> Extract for fn() -> T {
        type R = T;
    }
    pub type Never = <fn() -> ! as Extract>::R;
}
