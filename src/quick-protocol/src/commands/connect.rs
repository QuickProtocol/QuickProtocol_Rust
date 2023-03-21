use super::super::QpCommandInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    #[serde(rename = "InstructionIds")]
    pub instruction_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "BufferSize")]
    pub buffer_size: usize,
    #[serde(rename = "Question")]
    pub question: String,
}

impl Request {
    pub fn schema() -> QpCommandInfo {
        QpCommandInfo {
            name: String::from("连接"),
            description: String::default(),
            request_type_name: String::from("Quick.Protocol.Commands.Connect.Request"),
            request_type_schema: String::default(),
            request_type_schema_sample: String::default(),
            response_type_name: String::from("Quick.Protocol.Commands.Connect.Response"),
            response_type_schema: String::default(),
            response_type_schema_sample: String::default(),
        }
    }
}
