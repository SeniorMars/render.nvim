use std::fmt;
use std::path::PathBuf;
use std::{io, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidMath(String, String, usize), // reason, element, line
    InvalidDvisvgm(String),
    FileNotFound(PathBuf),
    // BinaryNotFound(which::Error),
    UnknownFence(String),
    InvalidImage(String),
    Io(io::Error),
    // ProtocolError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = match self {
            Error::InvalidMath(reason, element, line) => format!(
                "could not parse math {} at {} due to {}",
                element, line, reason
            ),
            Error::InvalidDvisvgm(err) => err.to_string(),
            Error::FileNotFound(path) => format!("could not find file {}", path.to_str().unwrap()),
            // Error::BinaryNotFound(binary) => format!("binary not found: {}", binary),
            Error::UnknownFence(kind) => format!("unknown fence with name {}", kind),
            Error::InvalidImage(path) => format!("could not read in {} as image", path),
            Error::Io(io_err) => format!("IO error: {}", io_err),
        };

        write!(f, "{}", res)
    }
}
