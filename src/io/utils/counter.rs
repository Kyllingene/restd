use crate::{fmt, io};

/// A sink that counts the number of bytes written through both [`fmt::Write`]
/// and [`io::Write`].
///
/// Doesn't actually write any data anywhere.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Counter(pub usize);

impl Counter {
    /// Create a new, empty counter.
    pub fn new() -> Self {
        Self(0)
    }
}

impl io::Write for Counter {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.0 += data.len();
        Ok(data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl fmt::Write for Counter {
    fn write_str(&mut self, data: &str) -> fmt::Result {
        self.0 += data.chars().count();
        Ok(())
    }

    fn write_char(&mut self, _: char) -> fmt::Result {
        self.0 += 1;
        Ok(())
    }
}
