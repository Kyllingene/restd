use crate::io::{Read, ReadResult, Write};

pub mod buffer;
pub mod counter;
pub mod cursor;
pub mod io_fmt;

/// Copy bytes from `reader` to `writer` until `reader` is exhausted (returns
/// `None`).
pub fn copy<R, W>(reader: &mut R, writer: &mut W) -> ReadResult
where
    R: Read + ?Sized,
    W: Write + ?Sized,
{
    let mut any = false;
    let mut total = 0;
    let mut buf = [0; 128];
    while let Some(read) = reader.read(&mut buf)? {
        writer.write_all(&buf)?;
        total += read;
        any = true;
    }

    if any {
        Ok(Some(total))
    } else {
        Ok(None)
    }
}
