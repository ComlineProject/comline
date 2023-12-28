// Generated with Comline compiler and code generator

pub trait Context {
	fn context(&self) -> Self;
}
pub struct Capability(String);

#[allow(async_fn_in_trait)]
pub trait HealthCheckProvider {
	fn alive(&self);
	fn capabilities(&self) -> Vec<Capability>;
}

#[allow(async_fn_in_trait)]
pub trait HealthCheckConsumer {
	fn alive(&self);
	fn capabilities(&self);
}
