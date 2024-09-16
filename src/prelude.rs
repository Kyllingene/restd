//! Things such as you might find in the std prelude; mainly macros.

pub use crate::{format_args, format_args_nl, write, writeln};

#[cfg(any(feature = "std", test))]
pub use crate::{dbg, eprint, eprintln, format, print, println};
