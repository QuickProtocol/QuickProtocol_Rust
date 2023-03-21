use super::QpInstruction;
use super::QpNoticeInfo;
use super::QpCommandInfo;

impl QpInstruction{
    pub fn new(
        id:String,
        name:String,
        notice_infos:Vec<QpNoticeInfo>,
        command_infos:Vec<QpCommandInfo>
    )->QpInstruction{
        QpInstruction{
            id:id,
            name:name,
            notice_infos : notice_infos,
            command_infos: command_infos,
        }
    }

    pub fn base()->QpInstruction{
        let mut notice_infos = Vec::new();
        let command_infos = Vec::new();

        notice_infos.push(super::notices::PrivateNotice::schema());

        self::QpInstruction::new(
            String::from("Quick.Protocol.Base"),
            String::from("基础指令集"),
            notice_infos,
            command_infos
        )
    }
}