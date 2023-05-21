use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    BadRequest,
    BadResponse,
    InvalidReading,
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Self {
        Error::BadRequest
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error::BadResponse
    }
}
