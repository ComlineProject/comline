// Standard Uses
use std::path::Path;

// Crate Uses
use crate::utils;

// External Uses
use comline::schema::ir::frozen::unit::{
    FrozenUnit as SchemaUnit, FrozenContextWhole as SchemaWhole
};
use eyre::Result;


#[allow(unused)]
pub fn to_mlua_api(path: &Path, schemas: &Vec<SchemaWhole>) -> Result<()> {
    let mut details = vec![];

    for schema in schemas {
        details.push(to_mlua_module_table(&schema))
    }

    Ok(())
}

#[allow(unused)]
pub fn to_mlua_api_module(path: &Path, schemas: &Vec<SchemaWhole>) -> String {
    let mut module = utils::generation_note("#");
    module += "use mlua::prelude::*\n\n";

    module += "#[mlua::lua_module]";
    module += "fn my_module(lua: &Lua) -> LuaResult<LuaTable> {\n";
    module += "\tlet exports = lua.create_table()?\n";

    for schema in schemas {
        let name = &schema.0.name;
        let code = to_mlua_module_table(schema);

        module += format!(
            "\texports.set(\"{name}\", lua.create_function({name}_table)?)?;\n",
        ).as_str();
    }

    module += "\tOk(exports)";
    module += "}";

    module
}

#[allow(unused)]
pub fn to_mlua_module_table(schema: &SchemaWhole) -> String {
    let table = String::new();

    let mut namespace = None;

    for node in schema.1.iter() {
        use SchemaUnit::*;
        match node {
            Namespace(n) => namespace = Some(n.clone()),
            Constant { docstring, name, kind_value } => {
                let mut constant= String::new();

                constant += name;

                if let Some(docstring) = docstring {
                    constant += &*format!("-- {docstring}");
                }

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

    table
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

