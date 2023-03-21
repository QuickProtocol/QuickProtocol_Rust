mod qp_notice_info;
mod notices;
mod qp_instruction;
mod qp_channel_options;

#[derive(Debug)]
pub struct QpNoticeInfo{
	pub name:String,
	pub description:String,
	pub notice_type_name:String,
	pub notice_type_schema:String,
	pub notice_type_schema_sample:String
}

#[derive(Debug)]
pub struct QpCommandInfo{
	pub name:String,
	pub description:String,
	pub request_type_name:String,
	pub request_type_schema:String,
	pub request_type_schema_sample:String,
	pub response_type_name:String,
	pub response_type_schema:String,
	pub response_type_schema_sample:String
}

#[derive(Debug)]
pub struct QpInstruction{
	pub id:String,
	pub name:String,
	pub notice_infos:Vec<QpNoticeInfo>,
	pub command_infos:Vec<QpCommandInfo>
}

#[derive(Debug)]
pub struct QpChannelOptions {
	internal_compress:bool,
	internal_encrypt:bool,
	internal_transport_timeout:usize,
	pub heart_beat_interval:usize,
	pub password:String,
	pub read_timeout : usize,
	pub write_timeout : usize
}