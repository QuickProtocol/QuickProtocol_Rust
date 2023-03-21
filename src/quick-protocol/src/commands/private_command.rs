use super::super::QpCommandInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    #[serde(rename = "Action")]
    pub action: String,
    #[serde(rename = "Content")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "Content")]
    pub content: String,
}

impl Request {
    pub fn schema() -> QpCommandInfo {
        QpCommandInfo {
            name: String::from("私有命令"),
            description: String::default(),
            request_type_name: String::from("Quick.Protocol.Commands.PrivateCommand.Request"),
            request_type_schema: String::default(),
            request_type_schema_sample: String::default(),
            response_type_name: String::from("Quick.Protocol.Commands.PrivateCommand.Response"),
            response_type_schema: String::default(),
            response_type_schema_sample: String::default(),
        }
    }
}
