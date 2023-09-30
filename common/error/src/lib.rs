use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HexagonalErrorCode {
    #[serde(rename = "NotFound")]
    NotFound,
    #[serde(rename = "Conflict")]
    Conflict,
    #[serde(rename = "BadRequest")]
    BadInput,
    #[serde(rename = "AdaptorError")]
    AdaptorError,
    #[serde(rename = "Unkown")]
    Unkown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HexagonalError {
    pub error: HexagonalErrorCode,
    pub message: String,
    #[serde(skip)]
    pub trace: String,
}

impl fmt::Display for HexagonalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Unable to perform operation, with error code: {:?}, and message: {} \n Trace: {}",
            self.error, self.message, self.trace
        )
    }
}

impl HexagonalErrorCode {
    pub fn map_to_http(&self) -> http::StatusCode {
        match self {
            HexagonalErrorCode::NotFound => http::StatusCode::NOT_FOUND,
            HexagonalErrorCode::Conflict => http::StatusCode::CONFLICT,
            HexagonalErrorCode::BadInput => http::StatusCode::BAD_REQUEST,
            HexagonalErrorCode::AdaptorError => http::StatusCode::INTERNAL_SERVER_ERROR,
            HexagonalErrorCode::Unkown => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HexagonalError {
    pub fn compile_to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn compile_to_http_response(&self) -> http::Response<String> {
        http::Response::builder()
            .status(self.error.map_to_http())
            .header("content-type", "application/json")
            .body(self.compile_to_json())
            .unwrap()
    }
}
