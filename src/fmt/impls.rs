use core::ops::DerefMut;

use super::{Debug, Display, Format, Result, Style, Write};

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
    S: Style + ?Sized,
{
    fn fmt(&self, f: &mut S) -> Result {
        (*self).fmt(f)
    }
}

impl Format<Debug> for () {
    fn fmt(&self, f: &mut Debug) -> Result {
        f.write_str("()")
    }
}

impl Format<Debug> for str {
    fn fmt(&self, f: &mut Debug) -> Result {
        f.write_char('"')?;
        for ch in self.chars() {
            if ch == '"' {
                f.write_str(r#"\"#)?;
            } else {
                dbg_char(ch, f.deref_mut())?;
            }
        }
        f.write_char('"')
    }
}

impl Format<Display> for str {
    fn fmt(&self, f: &mut Display) -> Result {
        f.write_str(self)
    }
}

impl Format<Debug> for char {
    fn fmt(&self, f: &mut Debug) -> Result {
        f.write_char('\'')?;
        dbg_char(*self, f.deref_mut())?;
        f.write_char('\'')?;

        Ok(())
    }
}

impl Format<Display> for char {
    fn fmt(&self, f: &mut Display) -> Result {
        f.write_char(*self)
    }
}
