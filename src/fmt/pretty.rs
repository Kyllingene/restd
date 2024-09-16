use super::{Format, Result, Style, Write};

/// A pretty-printing representation, like [`Debug`](super::Debug) but
/// emphasizing readability (e.g. spread over multiple lines).
///
/// The inner `usize` represents how many layers of indentation deep it
/// currently is.
///
/// Can be derived via [`derive`](crate::fmt::derive).
///
/// Equivalent to the alternate (`#`) form of [`core::fmt::Debug`].
#[derive(Default, Clone, Copy)]
pub struct Pretty(pub usize);
impl Style for Pretty {}
super::derive!(struct Pretty(depth));

impl Pretty {
    /// Creates a new `Pretty` at 0 indentation. Equivalent to `Pretty(0)`.
    pub const fn new() -> Self {
        Self(0)
    }

    /// Assists with formatting tuple structs (or tuples, when `name` is empty).
    pub fn dbg_tuple<'w>(&self, f: &'w mut dyn Write, name: &str) -> PrettyTuple<'w> {
        PrettyTuple::new(f, name, self.0)
    }

    /// Assists with formatting structs (or objects, when `name` is empty).
    pub fn dbg_struct<'w>(&self, f: &'w mut dyn Write, name: &str) -> PrettyStruct<'w> {
        PrettyStruct::new(f, name, self.0)
    }
}

/// A helper for formatting tuples and tuple structs.
///
/// If an error is encountered, any future calls will no-op.
pub struct PrettyTuple<'w> {
    f: &'w mut dyn Write,
    depth: usize,
    err: Result,
}

impl<'w> PrettyTuple<'w> {
    /// Create a new `PrettyTuple`. Leave `name` empty for formatting tuples.
    pub fn new(f: &'w mut dyn Write, name: &str, depth: usize) -> Self {
        let err = f.write_str(name).and_then(|_| f.write_str("("));

        Self { f, err, depth }
    }

    /// Format a field using [`Pretty`].
    pub fn field<T: Format<Pretty>>(&mut self, data: &T) -> &mut Self {
        let depth = self.depth + 1;
        self.field_with(|f| data.fmt(f, &Pretty(depth)))
    }

    /// Format a field using a given style.
    pub fn field_styled<T: Format<S>, S: Style>(&mut self, data: &T, style: &S) -> &mut Self {
        self.field_with(|f| data.fmt(f, style))
    }

    /// Format a field using a given closure instead of data.
    pub fn field_with(&mut self, f: impl FnOnce(&mut dyn Write) -> Result) -> &mut Self {
        self.newline(self.depth + 1);
        self.with_err(f);
        self.with_err(|f| f.write_char(','));
        self
    }

    /// Finish off the tuple (struct), returning an error if any were
    /// encountered.
    pub fn finish(&mut self) -> Result {
        self.newline(self.depth);
        self.with_err(|f| f.write_char(')'));
        self.err
    }

    /// Finish off the tuple (struct), with a trailing `...` field, returning an
    /// error if any were encountered.
    pub fn non_exhaustive(&mut self) -> Result {
        self.newline(self.depth + 1);
        self.with_err(|f| f.write_str("..."));
        self.newline(self.depth);
        self.with_err(|f| f.write_char(')'));

        self.err
    }

    /// Writes a newline, plus `depth` layers of indentation.
    fn newline(&mut self, depth: usize) {
        self.with_err(|f| {
            f.write_char('\n')?;
            for _ in 0..depth {
                f.write_str("    ")?;
            }
            Ok(())
        });
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
pub struct PrettyStruct<'w> {
    f: &'w mut dyn Write,
    depth: usize,
    err: Result,
}

impl<'w> PrettyStruct<'w> {
    /// Create a new `DebugStruct`. Leave `name` empty for formatting objects.
    pub fn new(f: &'w mut dyn Write, name: &str, depth: usize) -> Self {
        let err = f.write_str(name).and_then(|_| f.write_str(" {"));

        Self { f, depth, err }
    }

    /// Format a field using [`Pretty`].
    pub fn field<T: Format<Pretty>>(&mut self, name: &str, data: &T) -> &mut Self {
        let depth = self.depth + 1;
        self.field_with(name, |f| data.fmt(f, &Pretty(depth)))
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
        self.newline(self.depth + 1);
        self.with_err(|f| f.write_str(name).and_then(|_| f.write_str(": ")));
        self.with_err(f);
        self.with_err(|f| f.write_char(','));

        self
    }

    /// Finish off the struct (or object), returning an error if any were
    /// encountered.
    pub fn finish(&mut self) -> Result {
        self.newline(self.depth);
        self.with_err(|f| f.write_char('}'));
        self.err
    }

    /// Finish off the struct (or object), with a trailing `...` field,
    /// returning an error if any were encountered.
    pub fn non_exhaustive(&mut self) -> Result {
        self.newline(self.depth + 1);
        self.with_err(|f| f.write_str("..."));
        self.newline(self.depth);
        self.with_err(|f| f.write_char('}'));
        self.err
    }

    /// Writes a newline, plus `depth` layers of indentation.
    fn newline(&mut self, depth: usize) {
        self.with_err(|f| {
            f.write_char('\n')?;
            for _ in 0..depth {
                f.write_str("    ")?;
            }
            Ok(())
        });
    }

    /// Run a closure with the writer, if no errors have already been
    /// encountered, updating `self.err`.
    fn with_err(&mut self, f: impl FnOnce(&mut dyn Write) -> Result) {
        self.err = self.err.and_then(|_| f(self.f));
    }
}
