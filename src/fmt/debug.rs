use super::{Format, Result, Style, Write};

pub struct Debug;
impl Style for Debug {}

impl Debug {
    pub fn dbg_tuple<'w>(&self, f: &'w mut dyn Write, name: &str) -> DebugTuple<'w> {
        DebugTuple::new(f, name)
    }

    pub fn dbg_struct<'w>(&self, f: &'w mut dyn Write, name: &str) -> DebugStruct<'w> {
        DebugStruct::new(f, name)
    }
}

pub struct DebugTuple<'w> {
    f: &'w mut dyn Write,
    first: bool,
    err: Result,
}

impl<'w> DebugTuple<'w> {
    pub fn new(f: &'w mut dyn Write, name: &str) -> Self {
        let err = f.write_str(name).and_then(|_| f.write_char('('));

        Self {
            f,
            first: true,
            err,
        }
    }

    pub fn field<T: Format<Debug>>(&mut self, data: &T) -> &mut Self {
        self.field_with(|f| data.fmt(f, &Debug))
    }

    pub fn field_styled<T: Format<S>, S: Style>(&mut self, data: &T, style: &S) -> &mut Self {
        self.field_with(|f| data.fmt(f, style))
    }

    pub fn field_with(&mut self, f: impl FnOnce(&mut dyn Write) -> Result) -> &mut Self {
        if !self.first {
            self.with_err(|f| f.write_str(", "));
        }

        self.with_err(f);
        self.first = false;

        self
    }

    pub fn finish(&mut self) -> Result {
        self.err.and_then(|_| self.f.write_char(')'))
    }

    pub fn non_exhaustive(&mut self) -> Result {
        if !self.first {
            self.with_err(|f| f.write_str(", "));
        }

        self.err.and_then(|_| self.f.write_str("...)"))
    }

    fn with_err(&mut self, f: impl FnOnce(&mut dyn Write) -> Result) {
        self.err = self.err.and_then(|_| f(self.f));
    }
}

pub struct DebugStruct<'w> {
    f: &'w mut dyn Write,
    first: bool,
    err: Result,
}

impl<'w> DebugStruct<'w> {
    pub fn new(f: &'w mut dyn Write, name: &str) -> Self {
        let err = f.write_str(name).and_then(|_| f.write_str(" { "));

        Self {
            f,
            first: true,
            err,
        }
    }

    pub fn field<T: Format<Debug>>(&mut self, name: &str, data: &T) -> &mut Self {
        self.field_with(name, |f| data.fmt(f, &Debug))
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
        if !self.first {
            self.with_err(|f| f.write_str(", "));
        }

        self.with_err(|f| f.write_str(name).and_then(|_| f.write_str(": ")));
        self.with_err(f);
        self.first = false;

        self
    }

    pub fn finish(&mut self) -> Result {
        self.err.and_then(|_| self.f.write_str(" }"))
    }

    pub fn non_exhaustive(&mut self) -> Result {
        if !self.first {
            self.with_err(|f| f.write_str(", "));
        }

        self.err.and_then(|_| self.f.write_str("... }"))
    }

    fn with_err(&mut self, f: impl FnOnce(&mut dyn Write) -> Result) {
        self.err = self.err.and_then(|_| f(self.f));
    }
}
