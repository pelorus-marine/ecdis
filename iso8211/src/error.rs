use std::{error::Error, fmt, io::Error as IoError, num::ParseIntError, string::FromUtf8Error};

pub type Result<T> = std::result::Result<T, Iso8211Error>;

#[derive(Debug)]
pub enum Iso8211Error {
    IO(IoError),
    Parse(String),
    ParseInt(ParseIntError),
    ParseUtf8(FromUtf8Error),
}

impl fmt::Display for Iso8211Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Iso8211Error::IO(e) => write!(f, "I/O error: {}", e),
            Iso8211Error::ParseInt(e) => write!(f, "invalid numeric text in ISO 8211 field: {}", e),
            Iso8211Error::ParseUtf8(e) => write!(f, "invalid UTF-8 in ISO 8211 text: {}", e),
            Iso8211Error::Parse(s) => write!(f, "ISO 8211 parse error: {}", s),
        }
    }
}

impl Error for Iso8211Error {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Iso8211Error::IO(ref e) => Some(e),
            Iso8211Error::ParseInt(ref e) => Some(e),
            Iso8211Error::ParseUtf8(ref e) => Some(e),
            Iso8211Error::Parse(_) => None,
        }
    }
}

impl From<IoError> for Iso8211Error {
    fn from(err: IoError) -> Iso8211Error {
        Iso8211Error::IO(err)
    }
}

impl From<ParseIntError> for Iso8211Error {
    fn from(err: ParseIntError) -> Iso8211Error {
        Iso8211Error::ParseInt(err)
    }
}

impl From<FromUtf8Error> for Iso8211Error {
    fn from(err: FromUtf8Error) -> Iso8211Error {
        Iso8211Error::ParseUtf8(err)
    }
}
