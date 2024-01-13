// Standard Uses
use std::marker::PhantomData;

// Crate Uses

// Internal Uses
use comline_runtime::setup::call_system::CallSystem;

// External Uses
use mlua::prelude::*;
use mlua::UserData;


pub struct ConsumerSetupLua<T> {
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

pub fn setup_consumer_table(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    /*
    exports.set("with_transporter", lua.create_function(
        consumer_setup_with_transporter::<ConsumerSetupLua<_>>
    )?)?;
    */

    Ok(exports)
}


fn consumer_setup_with_transporter<T>(_: &Lua, transporter: T) -> LuaResult<ConsumerSetupLua<T>>
{
    Ok(ConsumerSetupLua { phantom: Default::default() })
}
