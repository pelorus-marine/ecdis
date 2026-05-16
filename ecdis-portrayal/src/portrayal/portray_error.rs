use std::fmt;

/// Errors from portrayal preparation (stub).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortrayError {
    UnsupportedScale,
}

impl fmt::Display for PortrayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedScale => write!(f, "unsupported display scale"),
        }
    }
}

impl std::error::Error for PortrayError {}
