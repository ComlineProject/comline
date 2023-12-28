// Generated with Comline compiler and code generator

// Standard Uses

// Crate Uses

// External Uses
use comline_runtime::setup::APIResult;


pub trait Context {
	fn context(&self) -> Self;
}
pub struct Capability(pub String);


#[allow(async_fn_in_trait)]
pub trait HealthCheckProtocol {
	fn alive(&self) -> APIResult<()>;
	fn capabilities(&self) -> APIResult<Vec<Capability>>;
}
