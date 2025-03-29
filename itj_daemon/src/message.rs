//TODO come up with a better name than this
pub trait DaemonDPK<TMsg> {
	fn serialize(msg: TMsg) -> Vec<u8>;
	fn deserialize(msg: &[u8]) -> TMsg;
	fn process(msg: &TMsg);
}
