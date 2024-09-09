use crate::{fmt, io};

/// An adapter from [`io::Write`] to [`fmt::Write`].
///
/// Flushes on every write.
pub struct IoFmt<W>(pub W);

impl<W: io::Write> fmt::Write for IoFmt<W> {
    fn write_str(&mut self, data: &str) -> fmt::Result {
        self.0.write(data.as_bytes()).and_then(|_| self.0.flush()).map(drop).map_err(|_| fmt::Error)
    }
}
