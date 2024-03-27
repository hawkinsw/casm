use std::error::Error as StdError;
use std::fmt::{Display, Debug};

pub enum Error {
    MachineFileParseError(String),
}

impl StdError for Error{}

impl Error {
    fn do_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MachineFileParseError(arg0) => f.debug_tuple("MachineFileParseError").field(arg0).finish(),
        }
    }
}

impl Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.do_fmt(f)
    }
}

impl Debug for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.do_fmt(f)
    }
}

