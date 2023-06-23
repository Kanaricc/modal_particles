use std::io::{Read, self, Write};


pub struct Buffer {
    inner: Vec<u8>,
}

impl Read for Buffer {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let size = self.inner.len();
        if size <= buf.len() {
            buf[..size].copy_from_slice(self.inner.as_slice());
            self.inner.clear();
            Ok(size)
        } else {
            buf.copy_from_slice(&self.inner[..buf.len()]);
            self.inner.drain(..buf.len());
            Ok(buf.len())
        }
    }
}

impl Write for Buffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Buffer {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn get_inner(&self)->&[u8]{
        &self.inner
    }

    pub fn clear(&mut self){
        self.inner.clear();
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}