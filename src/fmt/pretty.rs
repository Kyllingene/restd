use super::{Format, Result, Style, Write};

#[derive(Default, Clone, Copy)]
pub struct Pretty(pub usize);
impl Style for Pretty {}

impl Pretty {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn dbg_tuple<'w>(&self, f: &'w mut dyn Write, name: &str) -> PrettyTuple<'w> {
        PrettyTuple::new(f, name, self.0)
    }

    pub fn dbg_struct<'w>(&self, f: &'w mut dyn Write, name: &str) -> PrettyStruct<'w> {
        PrettyStruct::new(f, name, self.0)
    }
}

pub struct PrettyTuple<'w> {
    f: &'w mut dyn Write,
    depth: usize,
    err: Result,
}

impl<'w> PrettyTuple<'w> {
    pub fn new(f: &'w mut dyn Write, name: &str, depth: usize) -> Self {
        let err = f.write_str(name).and_then(|_| f.write_str("("));

        Self { f, err, depth }
    }

    pub fn field<T: Format<Pretty>>(&mut self, data: &T) -> &mut Self {
        let depth = self.depth + 1;
        self.field_with(|f| data.fmt(f, &Pretty(depth)))
    }

    pub fn field_styled<T: Format<S>, S: Style>(&mut self, data: &T, style: &S) -> &mut Self {
        self.field_with(|f| data.fmt(f, style))
    }

    pub fn field_with(&mut self, f: impl FnOnce(&mut dyn Write) -> Result) -> &mut Self {
        self.newline(self.depth + 1);
        self.with_err(f);
        self.with_err(|f| f.write_char(','));
        self
    }

    pub fn finish(&mut self) -> Result {
        self.newline(self.depth);
        self.with_err(|f| f.write_char(')'));
        self.err
    }

    pub fn non_exhaustive(&mut self) -> Result {
        self.newline(self.depth + 1);
        self.with_err(|f| f.write_str("..."));
        self.newline(self.depth);
        self.with_err(|f| f.write_char(')'));

        self.err
    }

    fn newline(&mut self, depth: usize) {
        self.with_err(|f| {
            f.write_char('\n')?;
            for _ in 0..depth {
                f.write_str("    ")?;
            }
            Ok(())
        });
    }

    fn with_err(&mut self, f: impl FnOnce(&mut dyn Write) -> Result) {
        self.err = self.err.and_then(|_| f(self.f));
    }
}

pub struct PrettyStruct<'w> {
    f: &'w mut dyn Write,
    depth: usize,
    err: Result,
}

impl<'w> PrettyStruct<'w> {
    pub fn new(f: &'w mut dyn Write, name: &str, depth: usize) -> Self {
        let err = f.write_str(name).and_then(|_| f.write_str(" {"));

        Self { f, depth, err }
    }

    pub fn field<T: Format<Pretty>>(&mut self, name: &str, data: &T) -> &mut Self {
        let depth = self.depth + 1;
        self.field_with(name, |f| data.fmt(f, &Pretty(depth)))
    }

    pub fn field_styled<T: Format<S>, S: Style>(
        &mut self,
        name: &str,
        data: &T,
        style: &S,
    ) -> &mut Self {
        self.field_with(name, |f| data.fmt(f, style))
    }

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

    pub fn finish(&mut self) -> Result {
        self.newline(self.depth);
        self.with_err(|f| f.write_char('}'));
        self.err
    }

    pub fn non_exhaustive(&mut self) -> Result {
        self.newline(self.depth + 1);
        self.with_err(|f| f.write_str("..."));
        self.newline(self.depth);
        self.with_err(|f| f.write_char('}'));
        self.err
    }

    fn newline(&mut self, depth: usize) {
        self.with_err(|f| {
            f.write_char('\n')?;
            for _ in 0..depth {
                f.write_str("    ")?;
            }
            Ok(())
        });
    }

    fn with_err(&mut self, f: impl FnOnce(&mut dyn Write) -> Result) {
        self.err = self.err.and_then(|_| f(self.f));
    }
}
