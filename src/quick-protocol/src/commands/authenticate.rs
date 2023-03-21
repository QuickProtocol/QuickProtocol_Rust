use super::super::QpCommandInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    #[serde(rename = "Answer")]
    pub answer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {}

impl Request {
    pub fn schema() -> QpCommandInfo {
        QpCommandInfo {
            name: String::from("认证"),
            description: String::default(),
            request_type_name: String::from("Quick.Protocol.Commands.Authenticate.Request"),
            request_type_schema: String::default(),
            request_type_schema_sample: String::default(),
            response_type_name: String::from("Quick.Protocol.Commands.Authenticate.Response"),
            response_type_schema: String::default(),
            response_type_schema_sample: String::default(),
        }
    }
}
