use super::super::QpCommandInfo;
use super::super::QpInstruction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "Data")]
    pub data: Vec<QpInstruction>,
}

impl Request {
    pub fn schema() -> QpCommandInfo {
        QpCommandInfo {
            name: String::from("获取全部指令集信息"),
            description: String::default(),
            request_type_name: String::from("Quick.Protocol.Commands.GetQpInstructions.Request"),
            request_type_schema: String::default(),
            request_type_schema_sample: String::default(),
            response_type_name: String::from("Quick.Protocol.Commands.GetQpInstructions.Response"),
            response_type_schema: String::default(),
            response_type_schema_sample: String::default(),
        }
    }
}
