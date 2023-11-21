// Relative Modules
pub mod schemas;
pub mod types;

// Standard Uses

// Crate Uses

// External Uses
use mlua::prelude::*;


#[mlua::lua_module]
fn test_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("schemas", schemas::schemas_table(lua)?)?;

    Ok(exports)
}

