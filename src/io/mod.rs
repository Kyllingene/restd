//! Core implementation for I/O operations.
//!
//! The central traits are [`Read`] and [`Write`].
//!
//! Reimplementation of
//! [`std::io`](https:?/doc.rust-lang.org/std/io/index.html).

use core::str;

mod impls;
mod utils;

#[cfg(test)]
mod test;

#[cfg(any(feature = "std", test))]
pub use impls::to_io;
pub use utils::{buffer::Buffer, copy, cursor::Cursor};

/// A specialized Result alias for I/O operations.
pub type Result<T> = core::result::Result<T, Error>;

// TODO: improve these docs
/// The result of a [read](Read) operation.
///
/// - None means no data was available.
/// - Some(0) means no data was available, but may be later.
/// - Some(n) means N bytes were read.
pub type ReadResult = Result<Option<usize>>;

/// A source for reading binary data.
///
/// Reimplementation of
/// [`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html).
pub trait Read {
    /// Reads some bytes into `buf`, returning the amount written.
    ///
    /// There's no guarantee on how many bytes are read; see
    /// [`read_exact`](Read::read_exact) if you need to know.
    fn read(&mut self, buf: &mut [u8]) -> ReadResult;

    /// Reads exactly enough bytes to fill `buf`, returning `Err(OutOfData)` if
    /// it couldn't.
    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
        while let Some(i) = self.read(buf)? {
            buf = &mut buf[i..];
            if buf.is_empty() {
                break;
            }
        }

        if !buf.is_empty() {
            Err(Error::OutOfData)
        } else {
            Ok(())
        }
    }

    /// Read some bytes onto the end of `buf`, returning the amount written.
    ///
    /// May reserve an unspecified (but small) amount of space if there's not
    /// enough.
    #[cfg(any(feature = "std", test))]
    fn read_onto_vec(&mut self, buf: &mut Vec<u8>) -> ReadResult {
        use core::mem;

        // TODO: arbitrary value
        if buf.capacity() - buf.len() < 16 {
            buf.reserve(16);
        }

        let uninit = buf.spare_capacity_mut();
        uninit.fill(mem::MaybeUninit::new(0));

        // SAFETY: the memory is initialized with 0s
        let target = unsafe { mem::transmute::<&mut [mem::MaybeUninit<u8>], &mut [u8]>(uninit) };
        let Some(read) = self.read(target)? else {
            // Returning here is ok since we've not broken any invariants
            return Ok(None);
        };

        // SAFETY: the memory is initialized with data, and bounds checks ensure
        // the allocated space is large enough
        unsafe { buf.set_len(buf.len() + read) };

        Ok(Some(read))
    }

    /// Exhaust the reader, reading onto the end of `buf`.
    #[cfg(any(feature = "std", test))]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> ReadResult {
        let mut read = 0;
        while let Some(i) = self.read_onto_vec(buf)? {
            read += i;
        }

        Ok(Some(read))
    }

    // TODO: make this more efficient
    /// Exhaust the reader, reading onto the end of `buf`.
    ///
    /// Ensures returned data is valid UTF-8.
    #[cfg(any(feature = "std", test))]
    fn read_to_string(&mut self, buf: &mut String) -> ReadResult {
        // SAFETY: the contents are checked for UTF-8 before returning,
        // and emptied if invalid
        let vec = unsafe { buf.as_mut_vec() };
        let mut read = 0;
        while let Some(i) = self.read_onto_vec(vec)? {
            read += i;
        }

        if let Err(e) = str::from_utf8(vec) {
            let _ = vec.drain(e.valid_up_to()..); // remove invalid UTF-8
            return Err(Error::Utf8Error(e));
        }

        Ok(Some(read))
    }
}

/// A sink for binary data.
///
/// Reimplementation of
/// [`std::io::Write`](https://doc.rust-lang.org/std/io/trait.Write.html).
pub trait Write {
    /// Write some bytes from `data`.
    ///
    /// There's no guarantee on how many bytes are written; see
    /// [`write_all`](Write::write_all) if you need to know.
    fn write(&mut self, data: &[u8]) -> Result<usize>;

    /// Flush the writer's data to an underlying source.
    ///
    /// On many types, this is a no-op.
    fn flush(&mut self) -> Result<()>;

    /// Write the entirety of `data`.
    fn write_all(&mut self, mut data: &[u8]) -> Result<()> {
        while !data.is_empty() {
            let written = self.write(data)?;
            data = &data[written..];
        }

        Ok(())
    }
}

/// A [reader](Read) or [writer](Write) that can be moved to an arbitrary position.
///
/// Reimplementation of
/// [`std::io::Seek`](https://doc.rust-lang.org/std/io/trait.Seek.html).
pub trait Seek {
    /// Seek forwards or backwards by a certain number of bytes.
    fn seek(&mut self, by: isize) -> Result<usize>;

    /// Seek to a specific location in bytes.
    fn seek_to(&mut self, to: usize) -> Result<usize>;

    /// Seek backwards from the end a certain number of bytes.
    fn seek_from_end(&mut self, by: usize) -> Result<usize>;

    /// Seek to the beginning.
    fn rewind(&mut self) -> Result<()> {
        self.seek_to(0).map(drop)
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    NotFound,
    PermissionDenied,
    Refused,
    Reset,
    Unreachable,
    Aborted,
    Unavailable,
    AlreadyExists,
    Broken,
    InvalidOperation,
    InvalidInput,
    InvalidData,
    TimedOut,
    Interrupted,
    Unsupported,
    OutOfData,
    OutOfSpace,
    Utf8Error(str::Utf8Error),
    Other,
}
