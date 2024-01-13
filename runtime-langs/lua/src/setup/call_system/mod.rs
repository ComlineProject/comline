// Relative Modules
pub mod consumer;
pub mod provider;

// Standard Uses
use std::marker::PhantomData;

// Crate Uses

// Internal Uses
use comline_runtime::setup::call_system::CallSystem;
use comline_runtime::setup::transport::consumer::CommunicationConsumer;

// External Uses
use mlua::prelude::*;
use mlua::UserData;


struct LuaCC(Box<dyn CommunicationConsumer>);

impl<'lua> FromLua<'lua> for LuaCC {
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        todo!()
    }
}

pub struct ConsumerSetupLua<T: ?Sized> {
    phantom: PhantomData<T>
}

impl<T: for<'lua> FromLua<'lua> + CallSystem> UserData for ConsumerSetupLua<T> {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method(
            "with_call_system",
            |_, this, call_system: T|
                {
                    Ok(())
                });
    }
}

impl<'lua, T> FromLua<'lua> for ConsumerSetupLua<T> {
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        todo!()
    }
}

pub fn setup_table(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("with_transporter", lua.create_function(|_, t: LuaCC|
        consumer_setup_with_transporter::<dyn CommunicationConsumer>(lua, t)
    )?)?;

    Ok(exports)
}


fn consumer_setup_with_transporter<T: CommunicationConsumer + ?Sized>(_: &Lua, transporter: LuaCC)
    -> LuaResult<ConsumerSetupLua<T>>
{
    Ok(ConsumerSetupLua { phantom: Default::default() })
}
