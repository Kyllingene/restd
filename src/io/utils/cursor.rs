use crate::io::{Error, Read, ReadResult, Result, Seek, Write};

/// A moving cursor over a slice of bytes.
///
/// Reading and writing both move forward in the data: if you don't want this,
/// consider using [`Buffer`](crate::io::Buffer) instead.
pub struct Cursor<A> {
    data: A,
    idx: usize,
}

impl<A> Cursor<A>
where
    A: AsRef<[u8]>,
{
    /// Create a new cursor, starting at the beginning.
    pub fn new(data: A) -> Self {
        Self { data, idx: 0 }
    }

    /// Create a new cursor, starting partway through the data.
    pub fn new_at(data: A, idx: usize) -> Self {
        Self { data, idx }
    }

    /// Returns the amount of remaining data.
    pub fn len(&self) -> usize {
        self.remaining().len() - self.idx
    }

    /// Returns the current position in the data.
    pub fn position(&self) -> usize {
        self.idx
    }

    /// Returns whether or not the cursor is at the end of the data.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Consumes the cursor and returns the original data.
    pub fn into_inner(self) -> A {
        self.data
    }

    /// Returns a view into the remaining data.
    pub fn remaining(&self) -> &[u8] {
        &self.data.as_ref()[self.idx..]
    }

    /// Get a mutable reference to the underlying data.
    pub fn get_ref(&self) -> &A {
        &self.data
    }

    /// Get a mutable reference to the underlying data.
    pub fn get_mut(&mut self) -> &mut A {
        &mut self.data
    }
}

impl<A> Cursor<A>
where
    A: AsMut<[u8]>,
{
    /// Returns a mutable view into the remaining data.
    pub fn remaining_mut(&mut self) -> &mut [u8] {
        &mut self.data.as_mut()[self.idx..]
    }
}

impl<A> Seek for Cursor<A>
where
    A: AsRef<[u8]>,
{
    fn seek(&mut self, by: isize) -> Result<usize> {
        self.idx = self
            .idx
            .saturating_add_signed(by)
            .min(self.data.as_ref().len());
        Ok(self.idx)
    }

    fn seek_to(&mut self, to: usize) -> Result<usize> {
        self.idx = to.min(self.data.as_ref().len());
        Ok(self.idx)
    }

    fn seek_from_end(&mut self, by: usize) -> Result<usize> {
        self.idx = self.data.as_ref().len().saturating_sub(by);
        Ok(self.idx)
    }

    fn rewind(&mut self) -> Result<()> {
        self.idx = 0;
        Ok(())
    }
}

impl<A> Read for Cursor<A>
where
    A: AsRef<[u8]>,
{
    fn read(&mut self, buf: &mut [u8]) -> ReadResult {
        let data = &self.data.as_ref()[self.idx..];

        if !data.is_empty() {
            let cap = data.len().min(buf.len());
            buf[..cap].copy_from_slice(&data[..cap]);

            self.idx += cap;
            Ok(Some(cap))
        } else {
            Ok(None)
        }
    }
}

impl<A> Write for Cursor<A>
where
    A: AsMut<[u8]>,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let data = &mut self.data.as_mut()[self.idx..];

        if !data.is_empty() {
            let cap = data.len().min(buf.len());
            data[..cap].copy_from_slice(&buf[..cap]);

            self.idx += cap;
            Ok(cap)
        } else {
            Err(Error::OutOfSpace)
        }
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
