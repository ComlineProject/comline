// Generated with Comline compiler and code generator

// Standard Uses

// Crate Uses

// External Uses
use comline_runtime::setup::APIResult;


pub const LOW_PING_RATE: u16 = 20;


#[allow(async_fn_in_trait)]
pub trait PingProvider {
	async fn ping(&self) -> APIResult<()>;
	fn ping_limit(&self) -> APIResult<()>;
}

#[allow(async_fn_in_trait)]
pub trait PingConsumer {
	async fn ping(&self);
	fn ping_limit(&self);

}
