// Standard Uses

// Crate Uses
use crate::utils;

// External Uses
use comline::schema::ir::frozen::unit::{FrozenArgument, FrozenUnit};
use comline::schema::ir::compiler::interpreted::kind_search::KindValue;

use eyre::eyre;


#[allow(unused)]
pub fn frozen_to_code(units: &Vec<FrozenUnit>) -> String {
    let mut code = utils::generation_note("#");

    code += "\
        import typing\n\
        import ctypes\n\
        \n\n\
    ";

    for unit in units { code += &*unit_to_code(&unit); }

    code
}

#[allow(unused)]
fn unit_to_code(frozen_unit: &FrozenUnit) -> String {
    let mut code = String::new();

    use FrozenUnit::*;
    match frozen_unit {
        Namespace(_) => {}
        Import(_) => {}
        Constant { docstring, name, kind_value } => {
            let (kind_name, default_value) = kind_value.name_and_value();

            let Ok(kind_py) = from_frozen_kind_to_python_kind(kind_value) else {
                panic!("No appropriate kind was found for '{kind_name}'")
            };

            // Name and kind
            code += &*format!("{name}: typing.Final[{kind_py}]");

            // Value
            if let Some(value) = default_value {
                code += &*format!(" = {value}");
            }

            code += "\n\n";
        }
        Property { .. } => {}
        Parameter { .. } => {}
        ExpressionBlock { .. } => {}
        Enum { .. } => {}
        EnumVariant(_) => {}
        Settings { .. } => {}
        Struct { .. } => {}
        Protocol {
            docstring, parameters,
            name, functions
        } => {
            // Async
            // TODO: Can we signify assyncness in python protocols, or if its that important, ABCs?
            //       Associated methods already can be marked "async"

            // Class Start
            code += &*format!("class {name}(typing.Protocol):\n");

            // Functions
            for function in functions {
                code += &*unit_to_code(function);
            }

            // Class End
            code += "\n";
        }
        Function {
            docstring, name, synchronous,
            arguments, _return,
            throws
        } => {
            // Async definition
            if !synchronous { code += "async "; }

            // Function Start
            code += &*format!("\tdef {name}(");

            // Arguments
            code += "self";

            let mut idx = 0;
            while idx != arguments.len() {
                let FrozenArgument { name, kind }
                    = arguments.get(idx).unwrap();

                let (kind_name, kind_value) = kind.name_and_value();
                code += &*format!("{}: {:?}", name, kind_name);

                if idx != arguments.len() { code += ", " }

                idx += 1;
            }

            // Arguments End
            code += ") ";

            // Returns
            if let Some(ret) = _return {
                let (ret_name, _) = ret.name_and_value();

                let Ok(kind_py) = from_frozen_kind_to_python_kind(ret) else {
                    panic!("No appropriate kind was found for '{ret_name}'")
                };

                code += &*format!("-> {kind_py}");
            }

            // Function Body
            code += ":\n\t\t...\n\n";

            // Function End
        }
        Error { .. } => {}
        Validator { .. } => {}
        _ => panic!("Reached forbidden or unimplemented node")
    }

    code
}

fn from_frozen_kind_to_python_kind(kind_value: &KindValue) -> eyre::Result<String> {
    let kind = match kind_value {
        KindValue::Primitive(primitive) => {
            let name = primitive.name();
            from_frozen_primitive_to_ctypes(name)
                .ok_or_else(|| eyre!("No primitive equivalent found for {name}"))?
        }
        KindValue::EnumVariant(_, _) => {
            todo!()
        }
        KindValue::Union(_) => {
            todo!()
        }
        KindValue::Namespaced(name, _) => {
            // TODO: Implement this properly, i was testing at this stage
            return Ok(name.clone())
        }
    };

    Ok(kind)
}

fn from_frozen_primitive_to_ctypes(primitive: &str) -> Option<String> {
    let kind = match primitive {
        "bool" => "c_bool",
        "u8" => "c_uint8",
        "u16" => "c_uint16",
        _ => { return None }
    };

    Some(format!("ctypes.{kind}"))
}
