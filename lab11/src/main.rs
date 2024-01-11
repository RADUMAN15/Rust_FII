use std::fs::File;
use std::io::{Result, Write};

struct MyWriter {
    inner: File,
}

impl MyWriter {
    fn new(file: File) -> MyWriter {
        MyWriter { inner: file }
    }
}

impl Write for MyWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut duplicated_buf = Vec::with_capacity(buf.len() * 2);

        for &byte in buf {
            duplicated_buf.push(byte);
            duplicated_buf.push(byte);
        }

        self.inner.write_all(&duplicated_buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}

fn main() -> Result<()> {
    let mut writer = MyWriter::new(File::create("a.txt")?);
    writer.write_all(b"abc")?;

    Ok(())
}
