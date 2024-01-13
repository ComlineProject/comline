// Relative Modules
mod setup;

// Standard Uses

// Crate Uses


// External Uses
use mlua::prelude::*;


#[mlua::lua_module]
fn rust_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("consumer_setup", setup::call_system::setup_table(lua)?)?;

    Ok(exports)
}

