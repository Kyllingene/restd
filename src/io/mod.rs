use core::str;

mod impls;
mod utils;

#[cfg(test)]
mod test;

#[cfg(any(feature = "std", test))]
pub use impls::to_io;
pub use utils::{copy, Buffer, Cursor};

pub type Result<T> = core::result::Result<T, Error>;
pub type ReadResult = Result<Option<usize>>;

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

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> ReadResult;

    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
        while let Some(i) = self.read(buf)? {
            buf = &mut buf[i..];
        }

        if !buf.is_empty() {
            Err(Error::OutOfData)
        } else {
            Ok(())
        }
    }

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
            return Ok(None);
        };

        if buf.len() + read > buf.capacity() {
            return Err(Error::OutOfSpace);
        }

        // SAFETY: the memory is initialized with data, and bounds checks ensure
        // the allocated space is large enough
        unsafe { buf.set_len(buf.len() + read) };

        Ok(Some(read))
    }

    #[cfg(any(feature = "std", test))]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> ReadResult {
        let mut read = 0;
        while let Some(i) = self.read_onto_vec(buf)? {
            read += i;
        }

        Ok(Some(read))
    }

    // TODO: make this more efficient
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

pub trait Write {
    fn write(&mut self, data: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, mut data: &[u8]) -> Result<()> {
        while !data.is_empty() {
            let written = self.write(data)?;
            data = &data[written..];
        }

        Ok(())
    }
}

pub trait Seek {
    fn seek(&mut self, by: isize) -> Result<usize>;
    fn seek_to(&mut self, to: usize) -> Result<usize>;
    fn seek_from_end(&mut self, by: usize) -> Result<usize>;

    fn rewind(&mut self) -> Result<()> {
        self.seek_to(0).map(|_| ())
    }
}
