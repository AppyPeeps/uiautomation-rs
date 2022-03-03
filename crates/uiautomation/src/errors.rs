use std::fmt::Display;

use windows::core::HRESULT;

pub const ERR_NONE: i32 = 0;
pub const ERR_NOTFOUND: i32 = 1;
pub const ERR_TIMEOUT: i32 = 2;

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    code: i32,
    message: String
}

impl Error {
    pub fn new(code: i32, message: &str) -> Error {
        Error {
            code,
            message: String::from(message)
        }
    }

    pub fn code(&self) -> i32 {
        self.code
    }

    pub fn result(&self) -> Option<HRESULT> {
        if self.code < 0 {
            Some(HRESULT(self.code))
        } else {
            None
        }
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {
}

impl From<windows::core::Error> for Error {
    fn from(e: windows::core::Error) -> Self {
        Self {
            code: e.code().0,
            message: e.message().to_string()
        }
    }
}

impl From<HRESULT> for Error {
    fn from(result: HRESULT) -> Self {
        Self {
            code: result.0,
            message: result.message().to_string()
        }
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error {
            code: 0,
            message
        }
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Error {
            code: 0,
            message: String::from(message)
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
