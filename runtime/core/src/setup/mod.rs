// Relative Modules
pub mod call_system;
pub mod message_format;
pub mod communication;
pub mod handler;


// TODO: A Error type will be necessary here, for things like API state and response status, information, etc
pub type APIResult<T> = Result<T, ()>;
