use crate::{fmt, io};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Counter(usize);

impl Counter {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn count(&self) -> usize {
        self.0
    }
    pub fn reset(&mut self) {
        self.0 = 0;
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
