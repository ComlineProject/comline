// Relative Modules
pub mod ping;
pub mod health;

// External Uses
use mlua::prelude::*;


pub(crate) fn schemas_table(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("ping", ping::ping_table(lua)?)?;
    exports.set("health", health::health_table(lua)?)?;

    Ok(exports)
}

