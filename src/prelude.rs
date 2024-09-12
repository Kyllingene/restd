pub use crate::{format_args, format_args_nl, write, writeln};

#[cfg(any(feature = "std", test))]
pub use crate::{dbg, eprint, eprintln, format, print, println};
