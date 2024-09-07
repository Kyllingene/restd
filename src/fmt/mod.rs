use core::ops::Deref;

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

pub trait Style {}

pub struct Formatter<'a, S: Style> {
    pub buf: &'a mut dyn Write,
    pub style: &'a S,
}

impl<'a, S: Style> Formatter<'a, S> {
    pub fn new(buf: &'a mut dyn Write, style: &'a S) -> Self {
        Self { buf, style }
    }
}

impl<S: Style> Deref for Formatter<'_, S> {
    type Target = S;

    fn deref(&self) -> &S {
        self.style
    }
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
    fn style<'a, S: Style>(&'a mut self, style: &'a S) -> Formatter<'a, S> {
        Formatter::new(self, style)
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
        $v:vis struct $name:ident
            $(< $($generics:tt)* >)?
            $(( $($tuple_fields:tt)* ))?
            $({ $($struct_fields:tt)* })?
            $(; $(@ $semicolon:tt)?)?

        $(impl {
            $($impl:tt)*
        })?
    ) => {
        $(#[$attr])*
        $v struct $name
            $(< $($generics)* >)?
            $(( $($tuple_fields)* ))?
            $({ $($struct_fields)* })?
            $(; $($semicolon)?)?

        impl $crate::fmt::Style for $name {}

        $(impl $name {
            $($impl)*
        })?
    };
}
use style;
