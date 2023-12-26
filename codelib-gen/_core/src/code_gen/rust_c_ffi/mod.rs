// Relative Modules
pub mod tests;

// Standard Uses
use std::path::Path;

// Crate Uses

// External Uses
use comline_core::schema;
use comline_core::schema::ir::frozen::unit::FrozenUnit as SchemaUnit;

use eyre::{Context, Result};


pub fn to_schemas_ffi(generation_path: &Path, schemas: &[Vec<SchemaUnit>]) -> Result<()> {
    let mut generated_code = vec![];

    for schema in schemas {
        let namespace = schema::ir::frozen::unit::schema_namespace_as_path(schema)
            .unwrap();

        generated_code.push((namespace, to_schema_code(schema)));
    }

    for (namespace, code) in generated_code {
        let schema_code_path = generation_path.join(format!("src/{}.rs", namespace));

        std::fs::create_dir_all(&schema_code_path.parent().unwrap()).with_context(|| {
            format!(
                "Could not create generated schema code directory at '{}'",
                schema_code_path.parent().unwrap().display()
            )
        })?;

        std::fs::write(&schema_code_path, code).with_context(|| {
            format!(
                "Could not write generated schema code file at '{}'",
                schema_code_path.display()
            )
        })?;
    }

    Ok(())
}

fn to_schema_code(schema_units: &[SchemaUnit]) -> String {
    let mut code = String::new();

    // let namespace = frozen::unit::namespace_or_panic(&schema.1).to_uppercase();

    for unit in schema_units {
        code += &from_unit_to_code(unit);
    }

    code
}


fn from_unit_to_code(unit: &SchemaUnit) -> String {
    use SchemaUnit::*;
    match unit {
        Namespace(namespace) => {
            format!("// Namespace {}\n\n", namespace)
        },
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
        Protocol { docstring, parameters, name, functions } => {
            let mut protocol = String::new();

            protocol += docstring;
            protocol += &*format!("pub mod {} {{\n", name);

            for function in functions {
                match function {
                    Function {
                        docstring, name, synchronous,
                        arguments, _return, throws
                    } => {
                        // TODO: Maybe turn protocol name into the camel_case format always
                        protocol += &*format!("\tpub extern \"C\" fn {}() {{\n", name);
                        protocol += "\t\ttodo!()\n";
                        protocol += "\t}\n\n";
                    },
                    _ => panic!("Condition shouldn't happen, report to the comline developers")
                }

            }

            protocol += "\n}\n";

            protocol
        }
        Field { docstring, parameters, name, kind_value, .. } => {
            let mut field = String::new();

            if let Some(doc) = docstring { field += &*make_docstring(doc) }
            let (kind_name, default_value) = kind_value.name_and_value();
            field += &*format!("{kind_name} {}", default_value.unwrap());

            field += "";

            field
        },
        missing => panic!("Missing implementation: '{:?}'", missing)
    }
}

fn make_docstring(doc: &str) -> String {
    format!(
        "///\n\
        ///{doc}\n\
        ///\n"
    )
}
