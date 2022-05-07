pub struct ErrorCode;

extern "C" {
	pub fn exit(exit_code: u8, ipc_offset: usize) -> !;
}

#[allow(non_upper_case_globals)]
impl ErrorCode {
	pub(crate) const IPCDataOverflow: u8 = 2;
	pub(crate) const DataInIPCNotValidUTF8: u8 = 3;
	pub(crate) const DeserializationError: u8 = 4;
	pub(crate) const SerializationError: u8 = 5;
}
