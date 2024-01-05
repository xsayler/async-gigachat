use std::time::SystemTimeError;

#[derive(Debug)]
pub enum GigaChatError {
    SystemError(String),
    StreamError(String),
    HttpError(String),
    JSONDeserializationError(String),
    InvalidArgument(String),
}

impl From<SystemTimeError> for GigaChatError {
    fn from(error: SystemTimeError) -> Self {
        GigaChatError::SystemError(error.to_string())
    }
}

impl From<reqwest::Error> for GigaChatError {
    fn from(error: reqwest::Error) -> Self {
        GigaChatError::HttpError(error.to_string())
    }
}

impl From<serde_json::Error> for GigaChatError {
    fn from(error: serde_json::Error) -> Self {
        GigaChatError::JSONDeserializationError(error.to_string())
    }
}

impl From<std::io::Error> for GigaChatError {
    fn from(error: std::io::Error) -> Self {
        GigaChatError::SystemError(error.to_string())
    }
}
