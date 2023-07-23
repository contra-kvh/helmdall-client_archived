use std::{
    error::Error,
    fmt::{self, Display, Formatter, Result},
    io::ErrorKind,
};

#[derive(Debug, Clone, Copy)]
pub enum APIErrorKind {
    APIAuthError,
    APIConnectionError,
}

#[derive(Debug)]
pub struct APIError {
    kind: APIErrorKind,
    message: String,
    code: i32,
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl APIError {
    pub fn new(kind: APIErrorKind, message: String, code: i32) -> APIError {
        APIError {
            kind,
            message,
            code,
        }
    }

    pub fn kind(&self) -> APIErrorKind {
        self.kind
    }

    pub fn code(&self) -> i32 {
        self.code
    }
}

impl Error for APIError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        &self.message
    }
}
