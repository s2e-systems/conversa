#![allow(clippy::too_many_arguments)]
#![allow(clippy::large_enum_variant)]

pub mod client;
pub mod multipart;
pub mod types;

use std::string::FromUtf8Error;

use reqwest::{Client, header::ToStrError};

#[derive(Debug)]
pub enum ConversaError {
    ClientError(String),
    InvalidData(String),
    IoError(String),
    UnexpectedStatusCode { code: u16, response: String },
    UnexpectedContentType(String),
    ErrorResponse(crate::types::ErrorResponse),
    Error(crate::types::Error),
}

impl From<reqwest::Error> for ConversaError {
    fn from(value: reqwest::Error) -> Self {
        ConversaError::ClientError(value.to_string())
    }
}

impl From<serde_json::Error> for ConversaError {
    fn from(value: serde_json::Error) -> Self {
        ConversaError::InvalidData(value.to_string())
    }
}

impl From<ToStrError> for ConversaError {
    fn from(value: ToStrError) -> Self {
        ConversaError::InvalidData(value.to_string())
    }
}

impl From<FromUtf8Error> for ConversaError {
    fn from(value: FromUtf8Error) -> Self {
        ConversaError::InvalidData(value.to_string())
    }
}

impl From<std::io::Error> for ConversaError {
    fn from(value: std::io::Error) -> Self {
        ConversaError::IoError(value.to_string())
    }
}

impl std::fmt::Display for ConversaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversaError::ClientError(msg) => write!(f, "Client error: {msg}",),
            ConversaError::InvalidData(msg) => write!(f, "Invalid data: {msg}",),
            ConversaError::UnexpectedStatusCode { code, response } => {
                write!(f, "Unexpected status code {code}: {response}",)
            }
            ConversaError::IoError(msg) => write!(f, "std::io error: {msg}",),
            ConversaError::UnexpectedContentType(content_type) => {
                write!(f, "Unexpected content type: {content_type}",)
            }
            ConversaError::ErrorResponse(err) => write!(f, "Error response: {err:?}",),
            ConversaError::Error(err) => write!(f, "Error: {err:?}",),
        }
    }
}
impl std::error::Error for ConversaError {}

pub type ConversaResult<T> = Result<T, ConversaError>;

pub struct OpenAIClientBuilder {
    api_key: String,
    address: String,
}

impl OpenAIClientBuilder {
    pub fn new(address: String, api_key: String) -> Self {
        Self { api_key, address }
    }

    pub fn build(self) -> ConversaResult<OpenAIClient> {
        let client = reqwest::ClientBuilder::new().build()?;
        Ok(OpenAIClient {
            client,
            api_key: self.api_key,
            base_address: self.address,
        })
    }
}

pub struct OpenAIClient {
    pub(crate) client: Client,
    pub(crate) api_key: String,
    pub(crate) base_address: String,
}
