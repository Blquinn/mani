use serde_tuple::{Serialize_tuple, Deserialize_tuple};
use serde::{Deserialize, Serialize};
use crate::base64;

#[derive(Debug, Deserialize_tuple, Serialize_tuple)]
pub struct ManiHeader {
    pub key: String,
    pub value: String
}

#[derive(Debug, Deserialize)]
pub struct ManiRequest {
    pub url: String,
    pub method: String,
    pub body_encoding: Option<String>,
    // body: Option<String>, // base64 encoded string
    #[serde(with = "base64::base64_opt")]
    pub body: Option<Vec<u8>>,
    pub headers: Vec<ManiHeader>
}

#[derive(Debug, Deserialize)]
pub struct ManiRequestWrapper {
    pub requests: Vec<ManiRequest>
}

#[derive(Debug, Serialize)]
pub struct ManiResponseMessage {
    pub status_code: u16,
    pub headers: Vec<ManiHeader>,
    #[serde(with = "base64::base64_opt")]
    pub body: Option<Vec<u8>>
}

#[derive(Debug, Serialize)]
pub struct ManiResponseError {
    pub description: String
}

#[derive(Debug, Serialize)]
pub struct ManiResponse {
    pub error: Option<ManiResponseError>,
    pub response: Option<ManiResponseMessage>
}

#[derive(Debug, Serialize)]
pub struct ManiResponseWrapper {
    pub responses: Vec<ManiResponse>
}