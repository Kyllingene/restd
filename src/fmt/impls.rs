use super::{Debug, Display, Format, Formatter, Result, Style, Write};

fn dbg_char(ch: char, f: &mut dyn Write) -> Result {
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
    fn fmt(&self, f: Formatter<'_, S>) -> Result {
        (*self).fmt(f)
    }
}

impl Format<Debug> for () {
    fn fmt(&self, mut f: Formatter<'_, Debug>) -> Result {
        f.write_str("()")
    }
}

impl Format<Debug> for str {
    fn fmt(&self, mut f: Formatter<'_, Debug>) -> Result {
        f.write_char('"')?;
        for ch in self.chars() {
            if ch == '"' {
                f.write_str(r#"\"#)?;
            } else {
                dbg_char(ch, &mut f)?;
            }
        }
        f.write_char('"')
    }
}

impl Format<Display> for str {
    fn fmt(&self, mut f: Formatter<'_, Display>) -> Result {
        f.write_str(self)
    }
}

impl Format<Debug> for char {
    fn fmt(&self, mut f: Formatter<'_, Debug>) -> Result {
        f.write_char('\'')?;
        dbg_char(*self, &mut f)?;
        f.write_char('\'')?;

        Ok(())
    }
}

impl Format<Display> for char {
    fn fmt(&self, mut f: Formatter<'_, Display>) -> Result {
        f.write_char(*self)
    }
}

#[cfg(any(test, feature = "std"))]
mod with_std {
    use crate::fmt::{Debug, Display, Format, Formatter, Result};

    impl Format<Debug> for String {
        fn fmt(&self, f: Formatter<'_, Debug>) -> Result {
            self.as_str().fmt(f)
        }
    }

    impl Format<Display> for String {
        fn fmt(&self, f: Formatter<'_, Display>) -> Result {
            self.as_str().fmt(f)
        }
    }
}
