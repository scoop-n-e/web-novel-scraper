use crate::api::common::error::{ApiError, Result};
use flate2::read::GzDecoder;
use serde::de::DeserializeOwned;
use std::io::Read;

#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Yaml,
    Json,
    Php,
    Atom,
    Jsonp(String),
}

impl OutputFormat {
    pub fn from_str(s: &str, callback: Option<&str>) -> Self {
        match s {
            "json" => OutputFormat::Json,
            "yaml" => OutputFormat::Yaml,
            "php" => OutputFormat::Php,
            "atom" => OutputFormat::Atom,
            "jsonp" => {
                if let Some(cb) = callback {
                    OutputFormat::Jsonp(cb.to_string())
                } else {
                    OutputFormat::Jsonp("callback".to_string())
                }
            }
            _ => OutputFormat::Yaml,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            OutputFormat::Yaml => "yaml".to_string(),
            OutputFormat::Json => "json".to_string(),
            OutputFormat::Php => "php".to_string(),
            OutputFormat::Atom => "atom".to_string(),
            OutputFormat::Jsonp(_) => "jsonp".to_string(),
        }
    }
}

pub trait ApiResponse: Sized + DeserializeOwned {
    fn from_json(data: &[u8]) -> Result<Self> {
        serde_json::from_slice(data).map_err(|e| ApiError::Deserialization(e.to_string()))
    }

    fn from_yaml(data: &[u8]) -> Result<Self> {
        serde_yaml::from_slice(data).map_err(|e| ApiError::Deserialization(e.to_string()))
    }
}

pub struct ResponseProcessor;

impl ResponseProcessor {
    pub fn decompress_gzip(data: &[u8]) -> Result<Vec<u8>> {
        let mut decoder = GzDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;
        Ok(decompressed)
    }

    pub fn process<T: ApiResponse>(
        data: Vec<u8>,
        format: &OutputFormat,
        gzip: bool,
    ) -> Result<T> {
        let decompressed = if gzip {
            Self::decompress_gzip(&data)?
        } else {
            data
        };

        match format {
            OutputFormat::Json => T::from_json(&decompressed),
            OutputFormat::Yaml => T::from_yaml(&decompressed),
            OutputFormat::Php => {
                Err(ApiError::InvalidFormat("PHP format is not supported".to_string()))
            }
            OutputFormat::Atom => {
                Err(ApiError::InvalidFormat("Atom format is not supported".to_string()))
            }
            OutputFormat::Jsonp(_) => {
                Err(ApiError::InvalidFormat("JSONP format is not supported".to_string()))
            }
        }
    }

    pub fn detect_format_from_content(data: &[u8]) -> OutputFormat {
        if data.is_empty() {
            return OutputFormat::Yaml;
        }

        let start = data.iter()
            .position(|&b| !b.is_ascii_whitespace())
            .unwrap_or(0);

        if start < data.len() {
            match data[start] {
                b'{' | b'[' => OutputFormat::Json,
                _ => OutputFormat::Yaml,
            }
        } else {
            OutputFormat::Yaml
        }
    }
}