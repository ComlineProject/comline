// Generated with Comline compiler and code generator

pub const LOW_PING_RATE: u16 = 20;


#[allow(async_fn_in_trait)]
pub trait PingProvider {
	async fn ping(&self);
	fn ping_limit(&self);
}

#[allow(async_fn_in_trait)]
pub trait PingConsumer {
	async fn ping(&self);
	fn ping_limit(&self);

}
