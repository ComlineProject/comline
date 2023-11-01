// Standard Uses
use std::path::Path;

// Crate Uses

// External Uses
use comline::schema::ir::frozen;
use comline::schema::ir::frozen::unit::{
    FrozenUnit as SchemaUnit,
    FrozenContextWhole as SchemaWhole
};
use eyre::Result;


#[allow(unused)]
pub fn to_api(path: &Path, schemas: &Vec<SchemaWhole>) -> Result<()> {
    let mut result = vec![];

    for schema in schemas {
        result.push(to_schema_code(schema));
    }

    Ok(())
}

fn to_schema_code(schema: &SchemaWhole) -> String {
    let mut code = String::new();

    let namespace = frozen::unit::namespace_or_panic(&schema.1).to_uppercase();

    code += &*format!("#ifndef {namespace}");
    code += &*format!("#define {namespace}");

    for unit in schema.1.iter() { code += &from_unit_to_code(unit); }

    code += &*format!("#endif // {namespace}");

    code
}

#[allow(unused)]
fn from_unit_to_code(unit: &SchemaUnit) -> String {
    let code = String::new();

    use SchemaUnit::*;
    match unit {
        Namespace(_) => {}
        Import(import) => {
            return format!("#include \"{import}\"")
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
        "//|\n\
        //!{doc}\n\
        //|\n"
    )
}

fn from_primitive_to_ctype(kind_name: &str) -> Option<String> {
    let kind = match kind_name {
        "bool" => "c_bool",
        "u8" => "c_uint8",
        "u16" => "c_uint16",
        _ => { return None }
    };

    Some(kind.to_owned())
}
