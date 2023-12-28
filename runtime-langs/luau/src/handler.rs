// Standard Uses
use std::path::Path;

// Crate Uses

// Internal Uses
use comline_runtime::package_abi::interface;
use comline_runtime::package_abi::interface::PackageLibRef;

// External Uses
use mlua::prelude::*;
use mlua::UserData;



pub(crate) struct Handler {
    package_lib: PackageLibRef
}

impl UserData for Handler {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("add_receiver", |_, this, ()| {
            Ok(())
        });
    }
}

pub(crate) fn handler_table(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("new", lua.create_function(handler_new)?)?;

    Ok(exports)
}


fn handler_new(_: &Lua, path: String) -> LuaResult<Handler> {
    let path = Path::new(&path);

    if !path.exists() { panic!("Package path '{:?}' doesn't exist", path) }
    if !path.is_dir() { panic!("Package path '{:?}' should be a directory", path) }

    let package_lib = interface::load_root_module(path)
        .unwrap_or_else(|e| panic!("Couldn't load package: {}", e));

    let handler = Handler { package_lib };

    Ok(handler)
}
