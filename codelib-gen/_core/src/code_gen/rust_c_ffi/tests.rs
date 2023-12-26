// Standard Uses
use std::path::Path;

// Crate Uses

// External Uses
use eyre::{Result, Context};
use heck::{ToLowerCamelCase, ToSnakeCase};

use comline_core::schema;
use comline_core::schema::ir::frozen::unit::{FrozenUnit as SchemaUnit, FrozenUnit};


pub fn to_schemas_tests(generation_path: &Path, schemas: &[Vec<SchemaUnit>]) -> Result<()> {
    let tests_path = generation_path.join("tests/");

    let mut generated_code = vec![];

    for schema in schemas {
        let namespace = schema::ir::frozen::unit::schema_namespace_as_path(schema)
            .unwrap();

        generated_code.push((namespace, to_schema_test_code(schema)));
    }

    for (namespace, code) in generated_code {
        let schema_code_path = tests_path.join(format!("{}.rs", namespace));

        std::fs::create_dir_all(&schema_code_path.parent().unwrap()).with_context(|| {
            format!(
                "Could not create generated schema test code directory at '{}'",
                schema_code_path.parent().unwrap().display()
            )
        })?;

        std::fs::write(&schema_code_path, code).with_context(|| {
            format!(
                "Could not write generated schema test code file at '{}'",
                schema_code_path.display()
            )
        })?;
    }

    Ok(())
}

pub fn to_schema_test_code(schema_units: &[SchemaUnit]) -> String {
    let mut code = String::new();

    for unit in schema_units {
        code += &from_unit_to_code(unit);
    }

    code
}


fn from_unit_to_code(unit: &SchemaUnit) -> String {
    use SchemaUnit::*;
    match unit {
        Namespace(ns) => {
            format!("// Namespace {ns}\n\n")
        }
        Protocol {
            docstring: _, parameters,
            name: pr_name, functions
        } => {
            let pr_name_camel = pr_name.to_snake_case();

            let mut protocol = String::new();
            protocol += "#[test]\n";
            protocol += &*format!("pub fn {}() {{\n", pr_name_camel);

            protocol += &*format!("\tlet {pr_name_camel} = {pr_name} {{}};\n\n");

            for function in functions {
                match function {
                    Function {
                        docstring: _, name: fn_name, synchronous: _,
                        arguments, _return, throws
                    } => {
                        protocol += &*format!("\t{pr_name_camel}.{fn_name}();\n");

                        // TODO: Check if the returned value is valid
                        // protocol += &*format!("\tassert_eq!({}, {})\n");
                    }
                    _ => panic!("Should only have Function units")
                }
            }

            protocol += "\n}\n\n";
            protocol
        }
        missing => panic!("Missing implementation for {:?}", missing)
    }
}

