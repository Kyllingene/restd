use core::ops::DerefMut;

mod args;
mod debug;
mod display;
mod impls;
mod macros;

#[cfg(test)]
mod test;

pub type Result = core::result::Result<(), Error>;

pub use debug::Debug;
pub use display::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Error;

pub trait Style: DerefMut<Target = dyn Write> {
    fn style(f: &mut dyn Write) -> &mut Self;
    fn done(&mut self) -> &mut dyn Write {
        self.deref_mut()
    }
}

pub trait Format<S: Style + ?Sized> {
    fn fmt(&self, f: &mut S) -> Result;
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
    fn style<S: Style + ?Sized>(&mut self) -> &mut S {
        S::style(self)
    }
}

impl<T: Write + Sealed> Stylable for T {}

use private::Sealed;
mod private {
    pub trait Sealed {}
    impl<T: super::Write> Sealed for T {}
}

#[macro_export]
macro_rules! style {
    (
        $(#[$attr:meta])*
        $v:vis struct $name:ident;
        $($impl:tt)*
    ) => {
        $(#[$attr])*
        #[repr(transparent)]
        $v struct $name(dyn $crate::fmt::Write);

        impl ::core::ops::Deref for $name {
            type Target = dyn $crate::fmt::Write;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::core::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl $crate::fmt::Style for $name {
            fn style(f: &mut dyn $crate::fmt::Write) -> &mut Self {
                // SAFETY: Self is repr(transparent)
                unsafe { ::core::mem::transmute(f) }
            }
        }

        impl $name {
            $($impl)*
        }
    };
}
use style;
