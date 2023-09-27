use std::fmt;

#[derive(Debug)]
pub enum HexagonalErrorCode {
    NotFound,
    Conflict,
    AdaptorError,
    Unkown
}

pub struct HexagonalError {
    pub error: HexagonalErrorCode,
    pub message: String,
    pub trace: String
}

impl fmt::Display for HexagonalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to perform model repository operation, with error code: {:?}, and message: {}", self.error, self.message)
    }
}