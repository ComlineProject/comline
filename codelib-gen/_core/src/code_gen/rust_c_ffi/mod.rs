// Standard Uses
use std::path::Path;

// Crate Uses

// External Uses
use comline_core::schema::ir::frozen::unit::{FrozenUnit as SchemaUnit};

use eyre::Result;


pub fn to_schemas_ffi(generation_path: &Path, schemas: &[Vec<SchemaUnit>]) -> Result<()> {
    let mut result = vec![];

    for schema in schemas {
        result.push(to_schema_code(schema));
    }

    for res in result {
        println!("Res: {res}")
    }

    Ok(())
}

fn to_schema_code(schema: &[SchemaUnit]) -> String {
    let mut code = String::new();

    // let namespace = frozen::unit::namespace_or_panic(&schema.1).to_uppercase();

    for unit in schema {
        code += &from_unit_to_code(unit);
    }

    code
}


fn from_unit_to_code(unit: &SchemaUnit) -> String {
    let code = String::new();

    use SchemaUnit::*;
    match unit {
        Namespace(_) => {}
        Import(import) => {
            return format!("use \"{import}\"")
        }
        Constant { docstring, name, kind_value } => {
            let mut constant = String::new();

            if let Some(doc) = docstring { constant += &*make_docstring(doc) }

            let (kind_name, default_value) = kind_value.name_and_value();
            constant += &*format!("const {kind_name} {}", default_value.unwrap());

            return constant
        }
        Property { .. } => {}
        Parameter { .. } => {}
        ExpressionBlock { .. } => {}
        Enum { .. } => {}
        EnumVariant(_) => {}
        Settings { .. } => {}
        Struct { docstring, parameters, name, fields } => {
            let mut structure = String::new();

            if let Some(doc) = docstring { structure += &*make_docstring(doc) }

            structure += &*format!("struct {name} {{");
            for field in fields {
                structure += &*from_unit_to_code(field);
            }
            structure += &*format!("}}");

            return structure
        }
        Protocol { .. } => {}
        Function { .. } => {}
        Error { .. } => {}
        Validator { .. } => {}
        Field { docstring, parameters, name, kind_value, .. } => {
            let mut field = String::new();

            if let Some(doc) = docstring { field += &*make_docstring(doc) }
            let (kind_name, default_value) = kind_value.name_and_value();
            field += &*format!("{kind_name} {}", default_value.unwrap());

            field += "";

            return field
        }
    }

    code
}

fn make_docstring(doc: &str) -> String {
    format!(
        "///\n\
        ///{doc}\n\
        ///\n"
    )
}

