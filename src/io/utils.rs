use super::{Read, ReadResult, Result, Seek, Write, Error};

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

pub struct Cursor<A> {
    data: A,
    idx: usize,
}

impl<A> Cursor<A> {
    pub fn new(data: A) -> Self {
        Self {
            data,
            idx: 0,
        }
    }
}

impl<A> Seek for Cursor<A>
where
    A: AsRef<[u8]>,
{
    fn seek(&mut self, by: isize) -> Result<usize> {
        self.idx = self.idx.saturating_add_signed(by).min(self.data.as_ref().len());
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

    fn flush(&mut self) -> Result<()> { Ok(()) }
}

pub struct Buffer<A> {
    data: A,
    len: usize,
}

impl<A> Buffer<A>
where
    A: AsRef<[u8]> + AsMut<[u8]>,
{
    pub fn new(data: A) -> Self {
        Self {
            data,
            len: 0,
        }
    }

    pub fn into_inner(self) -> A {
        self.data
    }

    pub fn data(&self) -> &[u8] {
        &self.data.as_ref()[..self.len]
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data.as_mut()[..self.len]
    }
}

impl<A> Read for Buffer<A>
where
    A: AsMut<[u8]>,
{
    fn read(&mut self, buf: &mut [u8]) -> ReadResult {
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

    fn flush(&mut self) -> Result<()> { Ok(()) }
}
