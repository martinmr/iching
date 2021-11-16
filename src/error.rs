#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    GenericError(String),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::GenericError("dummy message".to_string())
    }
}
