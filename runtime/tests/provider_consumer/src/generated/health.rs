// Generated with Comline compiler and code generator

// Standard Uses

// Crate Uses

// External Uses
use comline_runtime::setup::CallResult;


pub trait Context {
	fn context(&self) -> Self;
}
pub struct Capability(pub String);


#[allow(async_fn_in_trait)]
pub trait HealthCheckProtocol {
	fn alive(&self) -> CallResult<()>;
	fn capabilities(&self) -> CallResult<Vec<Capability>>;
}
