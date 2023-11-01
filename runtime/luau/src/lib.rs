// Relative Modules
pub(crate) mod handler;

// Standard Uses

// Crate Uses


// External Uses
use mlua::prelude::*;


#[mlua::lua_module]
fn rust_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("handler", handler::handler_table(lua)?)?;
    exports.set("runtime_info", runtime_info_table(lua)?)?;

    Ok(exports)
}


fn runtime_info_table(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("used_memory", lua.create_function(used_memory)?)?;

    Ok(exports)
}


fn used_memory(lua: &Lua, _: ()) -> LuaResult<usize> {
    Ok(lua.used_memory())
}
