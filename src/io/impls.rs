use super::{Read, ReadResult, Result, Write};

#[cfg(any(feature = "std", test))]
pub use with_std::to_io;

impl<T: Read> Read for &'_ mut T {
    fn read(&mut self, buf: &mut [u8]) -> ReadResult {
        (*self).read(buf)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        (*self).read_exact(buf)
    }

    #[cfg(any(feature = "std", test))]
    fn read_onto_vec(&mut self, buf: &mut Vec<u8>) -> ReadResult {
        (*self).read_onto_vec(buf)
    }

    #[cfg(any(feature = "std", test))]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> ReadResult {
        (*self).read_to_end(buf)
    }

    // TODO: make this more efficient
    #[cfg(any(feature = "std", test))]
    fn read_to_string(&mut self, buf: &mut String) -> ReadResult {
        (*self).read_to_string(buf)
    }
}

impl<T: Write> Write for &'_ mut T {
    fn write(&mut self, data: &[u8]) -> Result<usize> {
        (*self).write(data)
    }

    fn flush(&mut self) -> Result<()> {
        (*self).flush()
    }

    fn write_all(&mut self, data: &[u8]) -> Result<()> {
        (*self).write_all(data)
    }
}

#[cfg(any(feature = "std", test))]
mod with_std {
    use crate::io::{Error, Read, ReadResult, Result, Write};
    use std::io;

    impl Read for io::Stdin {
        fn read(&mut self, buf: &mut [u8]) -> ReadResult {
            match io::Read::read(self, buf) {
                Ok(0) => Ok(None),
                Ok(r) => Ok(Some(r)),
                Err(e) => to_io(e.kind()),
            }
        }
    }

    impl Read for io::StdinLock<'_> {
        fn read(&mut self, buf: &mut [u8]) -> ReadResult {
            match io::Read::read(self, buf) {
                Ok(0) => Ok(None),
                Ok(r) => Ok(Some(r)),
                Err(e) => to_io(e.kind()),
            }
        }
    }

    impl Write for io::Stdout {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            io::Write::write(self, buf).map_err(|e| from_write_err(e.kind()))
        }

        fn flush(&mut self) -> Result<()> {
            io::Write::flush(self).map_err(|e| from_write_err(e.kind()))
        }
    }

    impl Write for io::StdoutLock<'_> {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            io::Write::write(self, buf).map_err(|e| from_write_err(e.kind()))
        }

        fn flush(&mut self) -> Result<()> {
            io::Write::flush(self).map_err(|e| from_write_err(e.kind()))
        }
    }

    pub fn to_io(e: io::ErrorKind) -> ReadResult {
        Err(match e {
            io::ErrorKind::NotFound => Error::NotFound,
            io::ErrorKind::PermissionDenied => Error::PermissionDenied,
            io::ErrorKind::ConnectionRefused => Error::Refused,
            io::ErrorKind::ConnectionReset => Error::Reset,
            io::ErrorKind::ConnectionAborted => Error::Aborted,
            io::ErrorKind::NotConnected
            | io::ErrorKind::AddrInUse
            | io::ErrorKind::AddrNotAvailable
            | io::ErrorKind::BrokenPipe => Error::InvalidOperation,
            io::ErrorKind::AlreadyExists => Error::AlreadyExists,
            io::ErrorKind::WouldBlock => return Ok(Some(0)),
            io::ErrorKind::InvalidInput => Error::InvalidInput,
            io::ErrorKind::InvalidData => Error::InvalidData,
            io::ErrorKind::TimedOut => Error::TimedOut,
            io::ErrorKind::Interrupted => Error::Interrupted,
            io::ErrorKind::Unsupported => Error::Unsupported,
            io::ErrorKind::UnexpectedEof => Error::OutOfData,
            io::ErrorKind::OutOfMemory => Error::OutOfSpace,
            _ => Error::Other,
        })
    }

    pub fn from_write_err(e: io::ErrorKind) -> Error {
        match e {
            io::ErrorKind::NotFound => Error::NotFound,
            io::ErrorKind::PermissionDenied => Error::PermissionDenied,
            io::ErrorKind::ConnectionRefused => Error::Refused,
            io::ErrorKind::ConnectionReset => Error::Reset,
            io::ErrorKind::ConnectionAborted => Error::Aborted,
            io::ErrorKind::NotConnected
            | io::ErrorKind::AddrInUse
            | io::ErrorKind::AddrNotAvailable
            | io::ErrorKind::BrokenPipe => Error::InvalidOperation,
            io::ErrorKind::AlreadyExists => Error::AlreadyExists,
            io::ErrorKind::InvalidInput => Error::InvalidInput,
            io::ErrorKind::InvalidData => Error::InvalidData,
            io::ErrorKind::TimedOut => Error::TimedOut,
            io::ErrorKind::Interrupted => Error::Interrupted,
            io::ErrorKind::Unsupported => Error::Unsupported,
            io::ErrorKind::UnexpectedEof => Error::OutOfData,
            io::ErrorKind::OutOfMemory => Error::OutOfSpace,
            _ => Error::Other,
        }
    }
}
