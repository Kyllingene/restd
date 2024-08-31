use crate::io::{Error, Read, ReadResult, Result, Write};

/// A wrapper around an in-memory slice of bytes.
///
/// Reads consume from the start of the data, writes append.
pub struct Buffer<A> {
    data: A,
    len: usize,
}

impl<A> Buffer<A>
where
    A: AsRef<[u8]>,
{
    /// Create a new, empty buffer.
    ///
    /// If there's data inside you want to preserve, use
    /// [`new_with_len`](Buffer::new_with_len),
    /// [`new_full`](Buffer::new_full), or [`set_len`](Buffer::set_len).
    pub fn new(data: A) -> Self {
        Self { data, len: 0 }
    }

    /// Create a new buffer, preserving all existing data.
    pub fn new_full(data: A) -> Self {
        let len = data.as_ref().len();
        Self { data, len }
    }

    /// Wrap a slice of bytes, preserving `len` bytes of data.
    pub fn new_with_len(data: A, len: usize) -> Self {
        Self { data, len }
    }

    /// Override the used length of the buffer.
    ///
    /// Since the underlying slice is guaranteed to be initialized as far as
    /// safety is concerned, this is safe. However, be careful: when
    /// increasing the length, you may end up reading garbage data.
    pub fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    /// Returns the used length of the buffer.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the buffer is currently empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the remaining space in the buffer.
    pub fn cap(&self) -> usize {
        self.data().len() - self.len
    }

    /// Consumes the buffer and returns the original data.
    pub fn into_inner(self) -> A {
        self.data
    }

    /// Returns a view into the used portion of the buffer.
    pub fn data(&self) -> &[u8] {
        &self.data.as_ref()[..self.len]
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

impl<A> Buffer<A>
where
    A: AsMut<[u8]>,
{
    /// Returns a mutable view into the used portion of the buffer.
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data.as_mut()[..self.len]
    }
}

impl<A> Read for Buffer<A>
where
    A: AsMut<[u8]>,
{
    fn read(&mut self, buf: &mut [u8]) -> ReadResult {
        if buf.is_empty() {
            return Ok(Some(0));
        }

        let data = &mut self.data.as_mut()[..self.len];

        if !data.is_empty() {
            let cap = data.len().min(buf.len());
            buf[..cap].copy_from_slice(&data[..cap]);
            data.copy_within(cap.., 0);

            self.len -= cap;
            Ok(Some(cap))
        } else {
            Ok(None)
        }
    }
}

impl<A> Write for Buffer<A>
where
    A: AsMut<[u8]>,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let data = &mut self.data.as_mut()[self.len..];

        if !data.is_empty() {
            let cap = data.len().min(buf.len());
            data[..cap].copy_from_slice(&buf[..cap]);

            self.len += cap;
            Ok(cap)
        } else {
            Err(Error::OutOfSpace)
        }
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
