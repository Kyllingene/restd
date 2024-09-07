use core::marker::PhantomData;

use super::{Format, Formatter, Result, Style, Write};

pub struct Arguments<'a>(pub &'a [Var<'a>]);

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
    func: *const (),
    style: *const (),

    call: unsafe fn(
        *const (), // data
        *const (), // func
        *const (), // style
        &mut dyn Write,
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
            func: <T as Format<S>>::fmt as *const (),
            style: style as *const S as *const (),

            #[allow(warnings)]
            call: unsafe {
                core::mem::transmute(
                    call::<T, S>
                        as fn(&T, fn(&T, Formatter<'_, S>) -> Result, &S, &mut dyn Write) -> Result,
                )
            },

            _lt: PhantomData,
        }
    }

    pub fn call(&self, f: &mut dyn Write) -> Result {
        unsafe { (self.call)(self.data, self.func, self.style, f) }
    }
}

fn call<T, S>(
    data: &T,
    func: fn(&T, Formatter<'_, S>) -> Result,
    style: &S,
    buf: &mut dyn Write,
) -> Result
where
    S: Style,
{
    func(data, Formatter::new(buf, style))
}
