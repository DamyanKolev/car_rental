use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub data: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseWithStatus {
    pub status_code: u16,
    pub response: Response,
}
