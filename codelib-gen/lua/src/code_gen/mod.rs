// Standard Uses
use std::path::Path;

// Crate Uses

// External Uses
use comline_core::schema::ir::frozen::unit::{
    FrozenUnit as SchemaUnit, FrozenContextWhole as SchemaWhole
};
use comline_codelib_gen::utils;
use eyre::Result;


pub fn to_mlua_api(path: &Path, schemas: &Vec<SchemaWhole>) -> Result<()> {
    let mut details = vec![];

    for schema in schemas {
        details.push(to_mlua_module_table(&schema))
    }

    Ok(())
}

pub fn to_mlua_api_module(path: &Path, schemas: &Vec<SchemaWhole>) -> String {
    let mut module = utils::generation_note("#");
    module += "use mlua::prelude::*\n\n";

    module += "#[mlua::lua_module]";
    module += &*format!("fn {}(lua: &Lua) -> LuaResult<LuaTable> {{\n", "my_module");
    module += "\tlet exports = lua.create_table()?\n";

    for schema in schemas {
        let name = &schema.0.namespace_snake();
        let code = to_mlua_module_table(schema);

        module += format!(
            "\texports.set(\"{name}\", lua.create_function({name}_table)?)?;\n",
        ).as_str();
    }

    module += "\tOk(exports)";
    module += "}";

    module
}

fn to_mlua_module_table(schema: &SchemaWhole) -> String {
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

fn from_unit_to_code(unit: &SchemaUnit) -> String {
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

