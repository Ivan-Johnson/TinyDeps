pub trait MessageSerializer<TMsg> {
	fn serialize(msg: &TMsg) -> Vec<u8>;
	fn deserialize(msg: &[u8]) -> TMsg;
}
