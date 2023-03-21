use super::QpNoticeInfo;

pub struct PrivateNotice{
    pub action:String,
    pub content:String
}

impl PrivateNotice{
    pub fn schema()->QpNoticeInfo{
        QpNoticeInfo::new(
            String::from("Quick.Protocol.Notices.PrivateNotice"),
            String::from("私有通知"),
            String::from("用于传递私有协议通知。")
        )
    }
    pub fn new(action:String,content:String)->PrivateNotice{
        PrivateNotice{
            action,
            content
        }
    }
}
