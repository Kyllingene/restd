use super::{Format, Result, Style, Write};

/// A programmer-friendly repr for debugging purposes.
///
/// Often mimics the actual Rust structure of the type.
///
/// Can be derived via [`derive`](crate::fmt::derive).
///
/// Equivalent of [`core::fmt::Debug`].
#[derive(Default, Clone, Copy)]
pub struct Debug;
impl Style for Debug {}
super::derive!(struct Debug);

impl Debug {
    /// Assists with formatting tuple structs (or tuples, when `name` is empty).
    pub fn dbg_tuple<'w>(&self, f: &'w mut dyn Write, name: &str) -> DebugTuple<'w> {
        DebugTuple::new(f, name)
    }

    /// Assists with formatting structs (or objects, when `name` is empty).
    pub fn dbg_struct<'w>(&self, f: &'w mut dyn Write, name: &str) -> DebugStruct<'w> {
        DebugStruct::new(f, name)
    }
}

/// A helper for formatting tuples and tuple structs.
///
/// If an error is encountered, any future calls will no-op.
pub struct DebugTuple<'w> {
    f: &'w mut dyn Write,
    first: bool,
    err: Result,
}

impl<'w> DebugTuple<'w> {
    /// Create a new `DebugTuple`. Leave `name` empty for formatting tuples.
    pub fn new(f: &'w mut dyn Write, name: &str) -> Self {
        let err = f.write_str(name).and_then(|_| f.write_char('('));

        Self {
            f,
            first: true,
            err,
        }
    }

    /// Format a field using [`Debug`].
    pub fn field<T: Format<Debug>>(&mut self, data: &T) -> &mut Self {
        self.field_with(|f| data.fmt(f, &Debug))
    }

    /// Format a field using a given style.
    pub fn field_styled<T: Format<S>, S: Style>(&mut self, data: &T, style: &S) -> &mut Self {
        self.field_with(|f| data.fmt(f, style))
    }

    /// Format a field using a given closure instead of data.
    pub fn field_with(&mut self, f: impl FnOnce(&mut dyn Write) -> Result) -> &mut Self {
        if !self.first {
            self.with_err(|f| f.write_str(", "));
        }

        self.with_err(f);
        self.first = false;

        self
    }

    /// Finish off the tuple (struct), returning an error if any were
    /// encountered.
    pub fn finish(&mut self) -> Result {
        self.err.and_then(|_| self.f.write_char(')'))
    }

    /// Finish off the tuple (struct), with a trailing `...` field, returning an
    /// error if any were encountered.
    pub fn non_exhaustive(&mut self) -> Result {
        if !self.first {
            self.with_err(|f| f.write_str(", "));
        }

        self.err.and_then(|_| self.f.write_str("...)"))
    }

    /// Run a closure with the writer, if no errors have already been
    /// encountered, updating `self.err`.
    fn with_err(&mut self, f: impl FnOnce(&mut dyn Write) -> Result) {
        self.err = self.err.and_then(|_| f(self.f));
    }
}

/// A helper for formatting structs and objects.
///
/// If an error is encountered, any future calls will no-op.
pub struct DebugStruct<'w> {
    f: &'w mut dyn Write,
    first: bool,
    err: Result,
}

impl<'w> DebugStruct<'w> {
    /// Create a new `DebugStruct`. Leave `name` empty for formatting objects.
    pub fn new(f: &'w mut dyn Write, name: &str) -> Self {
        let err = f.write_str(name).and_then(|_| f.write_str(" { "));

        Self {
            f,
            first: true,
            err,
        }
    }

    /// Format a field using [`Debug`].
    pub fn field<T: Format<Debug>>(&mut self, name: &str, data: &T) -> &mut Self {
        self.field_with(name, |f| data.fmt(f, &Debug))
    }

    /// Format a field using a given style.
    pub fn field_styled<T: Format<S>, S: Style>(
        &mut self,
        name: &str,
        data: &T,
        style: &S,
    ) -> &mut Self {
        self.field_with(name, |f| data.fmt(f, style))
    }

    /// Format a field using a given closure instead of data.
    pub fn field_with(
        &mut self,
        name: &str,
        f: impl FnOnce(&mut dyn Write) -> Result,
    ) -> &mut Self {
        if !self.first {
            self.with_err(|f| f.write_str(", "));
        }

        self.with_err(|f| f.write_str(name).and_then(|_| f.write_str(": ")));
        self.with_err(f);
        self.first = false;

        self
    }

    /// Finish off the struct (or object), returning an error if any were
    /// encountered.
    pub fn finish(&mut self) -> Result {
        self.err.and_then(|_| self.f.write_str(" }"))
    }

    /// Finish off the struct (or object), with a trailing `...` field,
    /// returning an error if any were encountered.
    pub fn non_exhaustive(&mut self) -> Result {
        if !self.first {
            self.with_err(|f| f.write_str(", "));
        }

        self.err.and_then(|_| self.f.write_str("... }"))
    }

    /// Run a closure with the writer, if no errors have already been
    /// encountered, updating `self.err`.
    fn with_err(&mut self, f: impl FnOnce(&mut dyn Write) -> Result) {
        self.err = self.err.and_then(|_| f(self.f));
    }
}
