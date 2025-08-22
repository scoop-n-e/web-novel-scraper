use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    Network(reqwest::Error),
    Serialization(String),
    Deserialization(String),
    InvalidFormat(String),
    GzipDecompression(std::io::Error),
    Other(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Network(e) => write!(f, "Network error: {}", e),
            ApiError::Serialization(e) => write!(f, "Serialization error: {}", e),
            ApiError::Deserialization(e) => write!(f, "Deserialization error: {}", e),
            ApiError::InvalidFormat(format) => write!(f, "Invalid format: {}", format),
            ApiError::GzipDecompression(e) => write!(f, "Gzip decompression error: {}", e),
            ApiError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::Network(err)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::Deserialization(err.to_string())
    }
}

impl From<serde_yaml::Error> for ApiError {
    fn from(err: serde_yaml::Error) -> Self {
        ApiError::Deserialization(err.to_string())
    }
}

impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        ApiError::GzipDecompression(err)
    }
}

impl From<Box<dyn std::error::Error>> for ApiError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        ApiError::Other(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, ApiError>;