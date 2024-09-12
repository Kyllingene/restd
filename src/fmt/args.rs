use core::marker::PhantomData;

use super::{Format, Result, Style, Write};

pub struct Arguments<'a>(pub &'a [Var<'a>]);

type FmtFn<T, S> = fn(&T, &mut dyn Write, &S) -> Result;
type DynFmtFn = unsafe fn(
    *const (), // self
    &mut dyn Write,
    *const (), // style
) -> Result;

impl Arguments<'_> {
    pub fn write(&self, f: &mut dyn Write) -> Result {
        for var in self.0 {
            var.call(f)?;
        }

        Ok(())
    }
}

pub struct Var<'a> {
    data: *const (),
    style: *const (),
    func: DynFmtFn,

    _lt: PhantomData<&'a ()>,
}

impl<'a> Var<'a> {
    pub fn new<T, S>(data: &'a T, style: &'a S) -> Var<'a>
    where
        T: Format<S>,
        S: Style,
    {
        Self {
            data: data as *const T as *const (),
            style: style as *const S as *const (),

            // SAFETY: `*const ()` is ABI-compatible with `&T where T: Sized`
            func: unsafe {
                core::mem::transmute::<FmtFn<T, S>, DynFmtFn>(<T as Format<S>>::fmt as FmtFn<T, S>)
            },

            _lt: PhantomData,
        }
    }

    pub fn call(&self, f: &mut dyn Write) -> Result {
        unsafe { (self.func)(self.data, f, self.style) }
    }
}
