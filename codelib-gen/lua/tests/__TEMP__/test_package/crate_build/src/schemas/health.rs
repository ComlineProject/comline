// Standard Uses

// Crate Uses

// External Uses
use mlua::prelude::*;


pub(crate) fn health_table(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    /*
    exports.set("PingService", lua.create_function(|_, ()| -> LuaResult<PingService> {
        Ok(PingService::new())
    }))?;
    */

    Ok(exports)
}

