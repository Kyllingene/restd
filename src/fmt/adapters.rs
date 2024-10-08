use super::{Debug, Display, Error, Format, Pretty, Result, Write};
use core::fmt;

/// A wrapper around a type implementing [`core::fmt::Debug`] to make it
/// implement [`restd::fmt::Debug`](struct@Debug).
pub struct StdDebug<T>(pub T);

impl<T: fmt::Debug> Format<Debug> for StdDebug<T> {
    fn fmt(&self, f: &mut dyn Write, _: &Debug) -> Result {
        use fmt::Write as _;
        core::write!(RestdWrite(f), "{:?}", self.0).map_err(|_| Error)
    }
}

impl<T: fmt::Debug> Format<Pretty> for StdDebug<T> {
    fn fmt(&self, f: &mut dyn Write, _: &Pretty) -> Result {
        use fmt::Write as _;
        core::write!(RestdWrite(f), "{:#?}", self.0).map_err(|_| Error)
    }
}

/// A wrapper around a type implementing [`core::fmt::Display`] to make it
/// implement [`restd::fmt::Display`](Display).
pub struct StdDisplay<T>(pub T);
super::derive!(struct StdDisplay<T!>(t));

impl<T: fmt::Display> Format<Display> for StdDisplay<T> {
    fn fmt(&self, f: &mut dyn Write, _: &Display) -> Result {
        use fmt::Write as _;
        core::write!(RestdWrite(f), "{}", self.0).map_err(|_| Error)
    }
}

/// A wrapper around a type implementing [`core::fmt::Write`] to make it
/// implement [`restd::fmt::Write`](Write).
pub struct StdWrite<T>(pub T);
super::derive!(struct StdWrite<T!>(t));

impl<T: fmt::Write> Write for StdWrite<T> {
    fn write_str(&mut self, s: &str) -> Result {
        fmt::Write::write_str(&mut self.0, s).map_err(|_| Error)
    }

    fn write_char(&mut self, ch: char) -> Result {
        fmt::Write::write_char(&mut self.0, ch).map_err(|_| Error)
    }
}

/// A wrapper around a type implementing [`restd::fmt::Write`](Write) to make it
/// implement [`core::fmt::Write`].
pub struct RestdWrite<T>(pub T);
super::derive!(struct RestdWrite<T!>(t));

impl<T: Write> fmt::Write for RestdWrite<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        Write::write_str(&mut self.0, s).map_err(|_| fmt::Error)
    }

    fn write_char(&mut self, ch: char) -> fmt::Result {
        Write::write_char(&mut self.0, ch).map_err(|_| fmt::Error)
    }
}
