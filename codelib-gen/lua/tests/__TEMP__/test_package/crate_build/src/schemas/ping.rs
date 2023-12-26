// Standard Uses

// Crate Uses
use crate::types::{
    Service, Context,
    Parameters, Acceptance,
};

// External Uses
use mlua::prelude::*;
use mlua::UserData;


pub const LOW_PING_RATE: u16 = 20;


/// Ping another address
pub struct PingService {
    context: Context,
    ping_delegate: Option<LuaOwnedFunction>,
    ping_limit_delegate: Option<LuaOwnedFunction>,
}

impl PingService {
    pub(crate) fn new() -> Self {
        Self {
            context: Context {},
            ping_delegate: None,
            ping_limit_delegate: None
        }
    }
}

#[allow(unused)]
impl PingService {
    pub fn ping(&self, context: &Context) -> bool {
        let Some(ref cb) = self.ping_delegate else {
            panic!("A callback was not set for PingService:ping")
        };

        let result: bool = cb.call(()).unwrap_or_else(|e| {
            panic!("Callback error: {}", e)
        });

        result
    }
    pub fn ping_limit(&self, context: &Context) -> bool {
        todo!()
    }
}

impl Service for PingService {
    const PROVIDER_MODE: Acceptance = Acceptance::Multiple(Some(2));
    const CONSUMER_MODE: Acceptance = Acceptance::None;
}


impl Parameters for PingService {
    /*
    const PARAMETERS: [(usize, &'static str, ParameterValue); 1] = [
        (0, "timeout_ms", ParameterValue::U16(1000))
    ];
    */
}

impl UserData for PingService {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_set("ping", |_, this, value: LuaOwnedFunction| {
            Ok(this.ping_delegate = Some(value))
        });
        fields.add_field_method_set("ping_limit", |_, this, value: LuaOwnedFunction| {
            Ok(this.ping_limit_delegate = Some(value))
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("ping", |_, this, _:()| {
            Ok(this.ping(&this.context))
        })
    }
}


pub(crate) fn ping_table(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("PingService", lua.create_function(|_, _:()| {
        Ok(PingService::new())
    })?)?;

    Ok(exports)
}

