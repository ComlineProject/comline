// Generated with Comline compiler and code generator

// Standard Uses

// Crate Uses

// External Uses
use comline_runtime::setup::CallResult;


pub const LOW_PING_RATE: u16 = 20;


#[allow(async_fn_in_trait)]
pub trait PingProvider {
	async fn ping(&self) -> CallResult<()>;
	fn ping_limit(&self) -> CallResult<()>;
}

#[allow(async_fn_in_trait)]
pub trait PingConsumer {
	async fn ping(&self);
	fn ping_limit(&self);

}
