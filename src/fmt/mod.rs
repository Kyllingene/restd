mod adapters;
pub mod ansi;
pub mod args;
mod debug;
mod derives;
mod display;
mod hex;
mod impls;
mod macros;
mod pad;
mod prefix;
mod pretty;

#[cfg(test)]
mod test;

pub type Result = core::result::Result<(), Error>;

pub use adapters::{StdDebug, StdDisplay, StdWrite};
pub use debug::Debug;
pub use display::Display;
pub use hex::Hex;
pub use pad::{Dir, Pad};
pub use prefix::Prefix;
pub use pretty::Pretty;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Error;

pub trait Style {}
pub trait Modifier: Style {
    type Inner: Style;

    fn apply<T>(&self, f: &mut dyn Write, data: &T) -> Result
    where
        T: Format<Self::Inner> + ?Sized;
}

// TODO: can any form of this work?
// impl<T, M> Format<M> for T
// where
//     T: Format<M::Inner>,
//     M: Modifier,
// {
//     fn fmt(&self, f: &mut dyn Write, style: &M) -> Result {
//         style.apply(f, self)
//     }
// }

pub trait Format<S: Style> {
    fn fmt(&self, f: &mut dyn Write, style: &S) -> Result;

    #[cfg(any(feature = "std", test))]
    fn stringify(&self, style: &S) -> String {
        let mut f = String::new();
        self.fmt(&mut f, style).unwrap();
        f
    }
}

pub trait Write {
    fn write_str(&mut self, data: &str) -> Result;
    fn write_char(&mut self, data: char) -> Result {
        self.write_str(data.encode_utf8(&mut [0; 4]))
    }

    fn write_args(&mut self, args: args::Arguments<'_>) -> Result
    where
        Self: Sized,
    {
        args.write(self)
    }
}

impl Write for &mut dyn Write {
    fn write_str(&mut self, data: &str) -> Result {
        (*self).write_str(data)
    }

    fn write_char(&mut self, data: char) -> Result {
        (*self).write_char(data)
    }
}

impl<W: core::fmt::Write + ?Sized> Write for W {
    fn write_str(&mut self, data: &str) -> Result {
        self.write_str(data).map_err(|_| Error)
    }

    fn write_char(&mut self, data: char) -> Result {
        self.write_char(data).map_err(|_| Error)
    }
}

#[macro_export]
macro_rules! stylable {
    (for($($gen:tt)*) $($typ:tt)*) => {
        impl<
            __StylableModifier,
            $($gen)*
        > $crate::fmt::Format<__StylableModifier> for $($typ)*
        where
            $($typ)*: $crate::fmt::Format<__StylableModifier::Inner>,
            __StylableModifier: $crate::fmt::Modifier,
        {
            fn fmt(
                &self,
                f: &mut dyn $crate::fmt::Write,
                style: &__StylableModifier,
            ) -> $crate::fmt::Result {
                style.apply(f, self)
            }
        }
    };

    ($($typ:ty),+ $(,)?) => {$(
        impl<M> $crate::fmt::Format<M> for $typ
        where
            $typ: $crate::fmt::Format<M::Inner>,
            M: $crate::fmt::Modifier,
        {
            fn fmt(&self, f: &mut dyn $crate::fmt::Write, style: &M) -> $crate::fmt::Result {
                style.apply(f, self)
            }
        }
    )+};
}

#[doc(hidden)]
#[cfg(any(feature = "std", test))]
pub fn _print(args: args::Arguments<'_>) {
    crate::io::IoFmt(std::io::stdout())
        .write_args(args)
        .unwrap();
}

#[doc(hidden)]
#[cfg(any(feature = "std", test))]
pub fn _eprint(args: args::Arguments<'_>) {
    crate::io::IoFmt(std::io::stderr())
        .write_args(args)
        .unwrap();
}

#[doc(hidden)]
#[cfg(any(feature = "std", test))]
pub fn _format(args: args::Arguments<'_>) -> String {
    let mut s = String::new();
    args.write(&mut s).unwrap();
    s
}
