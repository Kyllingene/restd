pub mod args;
mod debug;
mod display;
mod hex;
mod impls;
mod macros;
mod pad;

#[cfg(test)]
mod test;

pub type Result = core::result::Result<(), Error>;

pub use debug::Debug;
pub use display::Display;
pub use pad::{Dir, Pad};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Error;

pub trait Style {}
pub trait Modifier: Style {
    type Inner: Style;

    fn apply<T>(&self, f: &mut dyn Write, data: &T) -> Result
    where
        T: Format<Self::Inner> + ?Sized;
}

// TODO: make this work
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
