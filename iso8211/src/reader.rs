use std::io::{BufReader, Read, Seek, SeekFrom};

use crate::{FIELD_TERMINATOR, Iso8211Error, Result, UNIT_TERMINATOR};

/// ISO 8211 file reader
pub struct Reader<T> {
    buffer: BufReader<T>,
}

impl<T: Read + Seek> Reader<T> {
    pub fn new(buffer: BufReader<T>) -> Reader<T> {
        Reader { buffer }
    }

    pub fn is_eof(&mut self) -> Result<bool> {
        let mut buf: [u8; 1] = [0; 1];
        let bytes = self.buffer.read(&mut buf)?;
        if bytes == 0 {
            Ok(true)
        } else {
            self.buffer.seek(SeekFrom::Current(-1))?;
            Ok(false)
        }
    }

    pub fn peek_byte(&mut self) -> Result<u8> {
        let result = self.read_u8();
        if result.is_ok() {
            self.buffer.seek(SeekFrom::Current(-1))?;
        }
        result
    }

    pub fn read_char(&mut self) -> Result<char> {
        let mut buf: [u8; 1] = [0; 1];
        self.buffer.read_exact(&mut buf)?;
        Ok(buf[0] as char)
    }

    pub fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = vec![0u8; length];
        self.buffer.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_str(&mut self, length: usize) -> Result<String> {
        let mut buf = vec![0u8; length];
        self.buffer.read_exact(&mut buf)?;
        let s = String::from_utf8(buf)?;
        Ok(s)
    }

    pub fn read_str_ft(&mut self) -> Result<String> {
        let mut buf: [u8; 1] = [0; 1];
        let mut bytes: Vec<u8> = Vec::new();
        while {
            self.buffer.read_exact(&mut buf)?;
            buf[0] != FIELD_TERMINATOR
        } {
            bytes.push(buf[0]);
        }
        let s = String::from_utf8(bytes)?;
        Ok(s)
    }

    pub fn read_str_ut(&mut self) -> Result<String> {
        let mut buf: [u8; 1] = [0; 1];
        let mut bytes: Vec<u8> = Vec::new();
        while {
            self.buffer.read_exact(&mut buf)?;
            buf[0] != UNIT_TERMINATOR
        } {
            bytes.push(buf[0]);
        }
        let s = String::from_utf8(bytes)?;
        Ok(s)
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.buffer.read_exact(&mut buf)?;
        let r = u8::from_le_bytes(buf);
        Ok(r)
    }

    pub fn read_u8_str(&mut self, length: usize) -> Result<u8> {
        let mut buf = vec![0u8; length];
        self.buffer.read_exact(&mut buf)?;
        let s = String::from_utf8(buf)?;
        let r = s.parse::<u8>()?;
        Ok(r)
    }

    pub fn read_u64(&mut self, length: usize) -> Result<u64> {
        let mut buf = vec![0u8; length];
        self.buffer.read_exact(&mut buf)?;
        let mut val = [0u8; 8];
        val[8 - length..].clone_from_slice(&buf);
        let r = u64::from_le_bytes(val);
        Ok(r)
    }

    pub fn read_u64_str(&mut self, length: usize) -> Result<u64> {
        let mut buf = vec![0u8; length];
        self.buffer.read_exact(&mut buf)?;
        let s = String::from_utf8(buf)?;

        match s.parse::<u64>() {
            Ok(r) => Ok(r),
            Err(_) => Err(Iso8211Error::Parse(format!(
                "error reading u64 {} byte string at position {}: '{}'",
                length,
                self.position()? - length as u64,
                s
            ))),
        }
    }

    /// Current read position in the underlying stream.
    pub fn position(&mut self) -> Result<u64> {
        Ok(self.buffer.stream_position()?)
    }
}
