#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    RequestError,
    ResponseError,
    InvalidDrawing,
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Self {
        Error::RequestError
    }
}
