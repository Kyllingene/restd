use core::marker::PhantomData;

use super::{Format, Result, Style, Write};

pub struct Arguments<'a, const N: usize>(pub [Var<'a>; N]);

impl<const N: usize> Arguments<'_, N> {
    pub fn write(&self, f: &mut dyn Write) -> Result {
        for var in &self.0 {
            var.call(f)?;
        }

        Ok(())
    }
}

pub struct Var<'a> {
    data: *const (),
    style: *const (),
    func: unsafe fn(
        *const (), // data
        &mut dyn Write,
        *const (), // style
    ) -> Result,

    _lt: PhantomData<&'a ()>,
}

impl<'a> Var<'a> {
    pub fn new<T, S>(
        data: &'a T,
        style: &'a S,
        // call: fn(&T, fn(&T, Formatter<'_, S>) -> Result, &S, &mut dyn Write) -> Result,
    ) -> Var<'a>
    where
        T: Format<S>,
        S: Style,
    {
        Self {
            data: data as *const T as *const (),
            style: style as *const S as *const (),

            #[allow(clippy::missing_transmute_annotations)]
            func: unsafe {
                core::mem::transmute(
                    <T as Format<S>>::fmt as fn(&T, &mut dyn Write, &S) -> Result,
                )
            },

            _lt: PhantomData,
        }
    }

    pub fn call(&self, f: &mut dyn Write) -> Result {
        unsafe { (self.func)(self.data, f, self.style) }
    }
}
