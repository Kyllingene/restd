use super::{Debug, Format, Pretty, Display, Result, Write, Error};
use core::fmt;

pub struct StdDebug<T>(pub T);

impl<T: fmt::Debug> Format<Debug> for StdDebug<T> {
    fn fmt(&self, f: &mut dyn Write, _: &Debug) -> Result {
        use fmt::Write as _;
        core::write!(StdWrite(f), "{:?}", self.0).map_err(|_| Error)
    }
}

impl<T: fmt::Debug> Format<Pretty> for StdDebug<T> {
    fn fmt(&self, f: &mut dyn Write, _: &Pretty) -> Result {
        use fmt::Write as _;
        core::write!(StdWrite(f), "{:#?}", self.0).map_err(|_| Error)
    }
}

pub struct StdDisplay<T>(pub T);

impl<T: fmt::Display> Format<Display> for StdDisplay<T> {
    fn fmt(&self, f: &mut dyn Write, _: &Display) -> Result {
        use fmt::Write as _;
        core::write!(StdWrite(f), "{}", self.0).map_err(|_| Error)
    }
}

pub struct StdWrite<T>(pub T);

impl<T: Write> fmt::Write for StdWrite<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        Write::write_str(&mut self.0, s).map_err(|_| fmt::Error)
    }

    fn write_char(&mut self, ch: char) -> fmt::Result {
        Write::write_char(&mut self.0, ch).map_err(|_| fmt::Error)
    }
}
