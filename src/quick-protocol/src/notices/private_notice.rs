use super::super::QpNoticeInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateNotice {
    #[serde(rename = "Action")]
    pub action: String,
    #[serde(rename = "Content")]
    pub content: String,
}

impl PrivateNotice {
    pub fn schema() -> QpNoticeInfo {
        QpNoticeInfo::new(
            String::from("Quick.Protocol.Notices.PrivateNotice"),
            String::from("私有通知"),
            String::from("用于传递私有协议通知。"),
        )
    }
}
