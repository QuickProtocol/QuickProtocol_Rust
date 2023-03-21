use serde::{Deserialize, Serialize};

pub mod notices;
pub mod commands;

mod qp_channel_options;
mod qp_instruction;
mod qp_notice_info;

#[derive(Debug, Serialize, Deserialize)]
pub struct QpNoticeInfo {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "NoticeTypeName")]
    pub notice_type_name: String,
    #[serde(rename = "NoticeTypeSchema")]
    pub notice_type_schema: String,
    #[serde(rename = "NoticeTypeSchemaSample")]
    pub notice_type_schema_sample: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QpCommandInfo {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "RequestTypeName")]
    pub request_type_name: String,
    #[serde(rename = "RequestTypeSchema")]
    pub request_type_schema: String,
    #[serde(rename = "RequestTypeSchemaSample")]
    pub request_type_schema_sample: String,
    #[serde(rename = "ResponseTypeName")]
    pub response_type_name: String,
    #[serde(rename = "ResponseTypeSchema")]
    pub response_type_schema: String,
    #[serde(rename = "ResponseTypeSchemaSample")]
    pub response_type_schema_sample: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QpInstruction {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "NoticeInfos")]
    pub notice_infos: Vec<QpNoticeInfo>,
    #[serde(rename = "CommandInfos")]
    pub command_infos: Vec<QpCommandInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QpChannelOptions {
    internal_compress: bool,
    internal_encrypt: bool,
    internal_transport_timeout: usize,
    #[serde(rename = "HeartBeatInterval")]
    pub heart_beat_interval: usize,
    #[serde(rename = "Password")]
    pub password: String,
    #[serde(rename = "ReadTimeout")]
    pub read_timeout: usize,
    #[serde(rename = "WriteTimeout")]
    pub write_timeout: usize,
}
