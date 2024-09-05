use core::ops::{Deref, DerefMut};

pub mod args;
mod debug;
mod display;
mod hex;
mod impls;
mod macros;

#[cfg(test)]
mod test;

pub type Result = core::result::Result<(), Error>;

pub use debug::Debug;
pub use display::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Error;

pub trait Style {
    type Arguments;
    fn create(args: &Self::Arguments) -> Self;
}

pub struct Formatter<'a, S: Style> {
    pub buf: &'a mut dyn Write,
    pub style: S,
}

impl<'a, S: Style> Formatter<'a, S> {
    pub fn new(buf: &'a mut dyn Write, args: &S::Arguments) -> Self {
        Self {
            buf,
            style: Style::create(args),
        }
    }

    pub fn inner(&mut self) -> &mut dyn Write {
        self.buf
    }

    pub fn into_inner(self) -> &'a mut dyn Write {
        self.buf
    }
}

impl<S: Style> Deref for Formatter<'_, S> {
    type Target = S;

    fn deref(&self) -> &S { &self.style }
}

impl<S: Style> DerefMut for Formatter<'_, S> {
    fn deref_mut(&mut self) -> &mut S { &mut self.style }
}

impl<S: Style> Write for Formatter<'_, S> {
    fn write_str(&mut self, data: &str) -> Result {
        self.buf.write_str(data)
    }

    fn write_char(&mut self, data: char) -> Result {
        self.buf.write_char(data)
    }
}

pub trait Format<S: Style> {
    fn fmt(&self, f: Formatter<'_, S>) -> Result;
}

pub trait Write {
    fn write_str(&mut self, data: &str) -> Result;
    fn write_char(&mut self, data: char) -> Result {
        self.write_str(data.encode_utf8(&mut [0; 4]))
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

pub trait Stylable: Write + Sized + Sealed {
    fn style<S: Style>(&mut self, args: &S::Arguments) -> Formatter<'_, S> {
        Formatter::new(self, args)
    }
}

impl<T: Write + Sealed> Stylable for T {}

use private::Sealed;
mod private {
    pub trait Sealed {}
    impl<T: super::Write> Sealed for T {}
}

// TODO: support style arguments
#[macro_export]
macro_rules! style {
    (
        $(#[$attr:meta])*
        $v:vis struct $name:ident;
        $($impl:tt)*
    ) => {
        $(#[$attr])*
        $v struct $name;

        impl $crate::fmt::Style for $name {
            type Arguments = ();
            fn create((): &()) -> Self { Self }
        }

        impl $name {
            $($impl)*
        }
    };
}
use style;
