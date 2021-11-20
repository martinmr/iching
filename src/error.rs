use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    RequestError,
    ResponseError,
    InvalidReading,
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Self {
        Error::RequestError
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error::ResponseError
    }
}

