// Standard Uses
use std::path::Path;

// Crate Uses

// External Uses
use comline_core::schema::ir::frozen::unit::{
    FrozenUnit as SchemaUnit, FrozenContextWhole as SchemaWhole
};
use comline_codelib_gen::utils;

use eyre::Result;


#[allow(unused)]
pub fn to_mlua_api(path: &Path, schemas: &Vec<SchemaWhole>) -> Result<()> {
    todo!()
}

#[allow(unused)]
pub fn to_mlua_api_module(path: &Path, schemas: Vec<String>) -> String {
    let mut module = format!("# {}\n\n", utils::generation_note("#"));
    module += "use mlua::prelude::*\n\n";

    module += "#[mlua::lua_module]";
    module += "fn my_module(lua: &Lua) -> LuaResult<LuaTable> {\n";
    module += "\tlet exports = lua.create_table()?\n";

    for schema in schemas {
        todo!()
        /*
        let (name, code) = to_mlua_module_table(schema);

        module += format!(
            "\texports.set(\"{name}\", lua.create_function({name}_table)?)?;\n",
        ).as_str();
        */
    }

    module += "\tOk(exports)";
    module += "}";

    module
}

#[allow(unused)]
pub fn to_mlua_module_table(schema: &SchemaWhole) -> (String, String) {
    let table = String::new();

    let mut namespace = None;

    for node in schema.1.iter() {
        use SchemaUnit::*;
        match node {
            Namespace(n) => namespace = Some(n.clone()),
            Constant { docstring, name, kind_value } => {
                let mut constant= String::new();

                constant += &*format!("lua.set(\"{}\", )", name);
            }
            Enum { .. } => {}
            Settings { .. } => {}
            Struct { .. } => {}
            Protocol { .. } => {}
            Error { .. } => {}
            Validator { .. } => {}
            _ => {}
        }
    }

    (namespace.unwrap(), table)
}

#[allow(unused)]
pub(crate) fn from_unit_to_code(unit: &SchemaUnit) -> String {
    let code = String::new();

    use SchemaUnit::*;
    match unit {
        Constant { docstring, name, kind_value } => {
            let mut constant= String::new();

            if let Some(docstring) = docstring {
                constant += &*format!("-- {docstring}");
            }
        }
        Property { .. } => {}
        Parameter { .. } => {}
        ExpressionBlock { .. } => {}
        Enum { .. } => {}
        EnumVariant(_) => {}
        Settings { .. } => {}
        Struct { .. } => {}
        Protocol { .. } => {}
        Function { .. } => {}
        Error { .. } => {}
        Validator { .. } => {}
        Field { .. } => {},
        _ => {}
    }

    code
}

