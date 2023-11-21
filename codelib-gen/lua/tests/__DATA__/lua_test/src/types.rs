// Standard Uses

// Crate Uses

// External Uses


pub struct Context {}
impl Context {
    /*
    pub(crate) fn execute<I, O>(
        &self, provider_mode: &Acceptance, consumer_mode: &Acceptance,
        f: F
    ) where I: FnOnce

    pub(crate) fn execute(
        &self, provider_mode: &Acceptance, consumer_mode: &Acceptance, f: &dyn FnOnce()
    ) {
        let a = f();
    }
    */
}

pub type Param = (fn(), ParameterValue);

pub trait Service {
    const PROVIDER_MODE: Acceptance;
    const CONSUMER_MODE: Acceptance;
}


pub trait Provider {}
pub trait Receiver {}

pub trait Parameters {
    // const PARAMETERS: [(usize, &'static str, ParameterValue)];
}

pub enum ParameterValue {
    U8(u8),
    U16(u16)
}


/// Tells how accepted are communications to a Protocol or Function
/// This is used to define the Provider or Consumer properties
pub enum Acceptance {
    None,
    Single,
    Multiple(Option<usize>)
}

