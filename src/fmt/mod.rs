use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

mod impls;
mod macros;
#[cfg(test)]
mod test;

pub type Result = core::result::Result<(), Error>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Error;

pub trait Style {} // TODO: is this anything more than a marker?
pub trait Format<S: Style> {
    fn fmt(&self, f: &mut Formatter<'_, S>) -> Result;
}

pub struct Formatter<'a, S> {
    buf: &'a mut dyn Write,
    _marker: PhantomData<fn() -> S>,
}

impl<'a, S> Formatter<'a, S>
where
    S: Style,
{
    #[doc(hidden)]
    pub fn new(buf: &'a mut impl Write) -> Self {
        Self {
            buf: buf as _,
            _marker: PhantomData,
        }
    }

    pub fn style<S2: Style>(&mut self) -> &mut Formatter<'a, S2> {
        // SAFETY: the only non-ZST field is `buf` which doesn't change
        unsafe { core::mem::transmute(self) }
    }
}

impl<'a, S> Deref for Formatter<'a, S> {
    type Target = &'a mut dyn Write;

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

impl<'a, S> DerefMut for Formatter<'a, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buf
    }
}

pub trait Write {
    fn write_str(&mut self, data: &str) -> Result;
    fn write_char(&mut self, data: char) -> Result {
        self.write_str(data.encode_utf8(&mut [0; 4]))
    }
}

pub enum Debug {}
pub enum Display {}
impl Style for Debug {}
impl Style for Display {}
