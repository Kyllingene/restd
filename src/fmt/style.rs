use super::{Write, Format, Result};

pub trait Foo {
    type IsModifier: Bool;
    type Style = u32;
}

pub trait Bool: Sealed {}

pub enum True {}
pub enum False {}

impl Sealed for True {}
impl Sealed for False {}
impl Bool for True {}
impl Bool for False {}

pub trait Style: Foo<IsModifier = False> {}
impl<S: Foo<IsModifier = False>> Style for S {}

pub trait Modifier: Foo<IsModifier = True> {
    type Inner: Foo;

    fn apply<T>(&self, f: &mut dyn Write, data: &T) -> Result
    where
        T: Format<Self::Inner> + ?Sized;
}

use private::Sealed;
mod private {
    pub trait Sealed {}
}
