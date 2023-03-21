use super::QpNoticeInfo;

impl QpNoticeInfo{
    pub fn new(name:String,description:String,notice_type_name:String)->QpNoticeInfo{
        QpNoticeInfo{
            name:name,
            description:description,
            notice_type_name:notice_type_name,
            notice_type_schema:String::default(),
            notice_type_schema_sample:String::default()
        }
    }
}