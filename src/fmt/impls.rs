use super::{Debug, Display, Format, Formatter, Result, Style};

fn dbg_char<S: Style>(ch: char, f: &mut Formatter<'_, S>) -> Result {
    if ch == '\'' {
        f.write_str("\\'")?;
    } else if ch.is_control() {
        todo!()
    } else {
        f.write_char(ch)?;
    }

    Ok(())
}

impl<'a, T, S> Format<S> for &'a T
where
    T: Format<S> + ?Sized,
    S: Style,
{
    fn fmt(&self, f: &mut Formatter<'_, S>) -> Result {
        (*self).fmt(f)
    }
}

impl Format<Debug> for () {
    fn fmt(&self, f: &mut Formatter<'_, Debug>) -> Result {
        f.write_str("()")
    }
}

impl Format<Debug> for str {
    fn fmt(&self, f: &mut Formatter<'_, Debug>) -> Result {
        f.write_char('"')?;
        for ch in self.chars() {
            if ch == '"' {
                f.write_str("\\\"")?;
            } else {
                dbg_char(ch, f)?;
            }
        }
        f.write_char('"')
    }
}

impl Format<Display> for str {
    fn fmt(&self, f: &mut Formatter<'_, Display>) -> Result {
        f.write_str(self)
    }
}

impl Format<Debug> for char {
    fn fmt(&self, f: &mut Formatter<'_, Debug>) -> Result {
        f.write_char('\'')?;
        dbg_char(*self, f)?;
        f.write_char('\'')?;

        Ok(())
    }
}

impl Format<Display> for char {
    fn fmt(&self, f: &mut Formatter<'_, Display>) -> Result {
        f.write_char(*self)
    }
}

#[cfg(any(feature = "std", test))]
mod with_std {
    use crate::fmt::{Error, Result, Write};

    impl Write for String {
        fn write_str(&mut self, data: &str) -> Result {
            core::fmt::Write::write_str(self, data).map_err(|_| Error)
        }

        fn write_char(&mut self, data: char) -> Result {
            core::fmt::Write::write_char(self, data).map_err(|_| Error)
        }
    }

    impl Write for std::io::Stdout {
        fn write_str(&mut self, data: &str) -> Result {
            use std::io::Write;

            self.write_all(data.as_bytes()).map_err(|_| Error)
        }
    }

    impl Write for std::io::Stderr {
        fn write_str(&mut self, data: &str) -> Result {
            use std::io::Write;

            self.write_all(data.as_bytes()).map_err(|_| Error)
        }
    }

    impl<'a> Write for std::io::StdoutLock<'a> {
        fn write_str(&mut self, data: &str) -> Result {
            use std::io::Write;

            self.write_all(data.as_bytes()).map_err(|_| Error)
        }
    }

    impl<'a> Write for std::io::StderrLock<'a> {
        fn write_str(&mut self, data: &str) -> Result {
            use std::io::Write;

            self.write_all(data.as_bytes()).map_err(|_| Error)
        }
    }
}
