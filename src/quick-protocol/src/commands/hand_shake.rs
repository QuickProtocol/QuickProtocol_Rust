use super::super::QpCommandInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    #[serde(rename = "TransportTimeout")]
    pub transport_timeout: usize,
    #[serde(rename = "EnableEncrypt")]
    pub enable_encrypt: bool,
    #[serde(rename = "EnableCompress")]
    pub enable_compress: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {}

impl Request {
    pub fn schema() -> QpCommandInfo {
        QpCommandInfo {
            name: String::from("握手"),
            description: String::default(),
            request_type_name: String::from("Quick.Protocol.Commands.HandShake.Request"),
            request_type_schema: String::default(),
            request_type_schema_sample: String::default(),
            response_type_name: String::from("Quick.Protocol.Commands.HandShake.Response"),
            response_type_schema: String::default(),
            response_type_schema_sample: String::default(),
        }
    }
}
