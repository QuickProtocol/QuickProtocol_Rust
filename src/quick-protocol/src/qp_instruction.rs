use super::QpCommandInfo;
use super::QpInstruction;
use super::QpNoticeInfo;

impl QpInstruction {
    pub fn new(
        id: String,
        name: String,
        notice_infos: Vec<QpNoticeInfo>,
        command_infos: Vec<QpCommandInfo>,
    ) -> QpInstruction {
        QpInstruction {
            id: id,
            name: name,
            notice_infos: notice_infos,
            command_infos: command_infos,
        }
    }

    pub fn base() -> QpInstruction {
        let mut notice_infos = Vec::new();
        let mut command_infos = Vec::new();

        notice_infos.push(super::notices::private_notice::PrivateNotice::schema());
        command_infos.push(super::commands::connect::Request::schema());
        command_infos.push(super::commands::authenticate::Request::schema());
        command_infos.push(super::commands::hand_shake::Request::schema());
        command_infos.push(super::commands::private_command::Request::schema());
        command_infos.push(super::commands::get_qp_instructions::Request::schema());

        self::QpInstruction::new(
            String::from("Quick.Protocol.Base"),
            String::from("基础指令集"),
            notice_infos,
            command_infos,
        )
    }
}
