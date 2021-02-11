use serde_tuple::{Serialize_tuple, Deserialize_tuple};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::base64;

#[derive(Debug, Deserialize_tuple, Serialize_tuple)]
pub struct ManiHeader {
    pub key: String,
    pub value: String
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Body {
    None,
    #[serde(with = "base64::base64")]
    Bytes(Vec<u8>),
    Json(Value)
}

impl Default for Body {
    fn default() -> Self {
        Body::None
    }
}

#[derive(Debug, Deserialize)]
pub struct ManiRequest {
    pub url: String,
    pub method: String,
    // TODO: Handle different body types better.
    pub body: Body,
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
    pub body: Body,
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