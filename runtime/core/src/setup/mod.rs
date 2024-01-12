// Relative Modules
pub mod communication;
pub mod call_system;
pub mod abstract_call;


// TODO: A Error type will be necessary here, for things like API Call state and response status, information, etc
pub type CallResult<T> = Result<T, ()>;


