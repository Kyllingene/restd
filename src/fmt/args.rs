//! Utilities for assembling a list of variables to format.

use core::marker::PhantomData;

use super::{Format, Result, Style, Write, Display};

/// A list of [`Var`]s.
pub struct Arguments<'a>(pub &'a [Var<'a>]);

type FmtFn<T, S> = fn(&T, &mut dyn Write, &S) -> Result;
type DynFmtFn = unsafe fn(
    *const (), // self
    &mut dyn Write,
    *const (), // style
) -> Result;

impl Arguments<'_> {
    /// Format each var into a writer sequentially.
    pub fn write(&self, f: &mut dyn Write) -> Result {
        for var in self.0 {
            var.call(f)?;
        }

        Ok(())
    }
}

crate::stylable!(for('a) Arguments<'a>);
impl Format<Display> for Arguments<'_> {
    fn fmt(&self, f: &mut dyn Write, _: &Display) -> Result {
        self.write(f)
    }
}

/// Essentially a vtable for a [`Format`]. Contains references to the data, the
/// style, and the data's method for formatting with that style.
pub struct Var<'a> {
    data: *const (),
    style: *const (),
    func: DynFmtFn,

    _lt: PhantomData<&'a ()>,
}

impl<'a> Var<'a> {
    /// Create a new `Var` wrapping `data` and `style`.
    ///
    /// Format using [`call`](Var::call).
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

    /// Format the type using the data stored by [`new`](Var::new).
    pub fn call(&self, f: &mut dyn Write) -> Result {
        unsafe { (self.func)(self.data, f, self.style) }
    }
}
