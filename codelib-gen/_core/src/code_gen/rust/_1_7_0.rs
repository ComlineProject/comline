// Standard Uses

// Crate Uses
use crate::utils;

// External Uses
use comline_core::schema::ir::frozen::unit::{
    FrozenArgument, FrozenUnit as FrozenSchemaUnit
};


pub fn frozen_schema_to_code(units: &Vec<FrozenSchemaUnit>) -> String {
    let mut code = utils::generation_note("//");

    for unit in units {
        code += &*unit_to_code(&unit);
    }

    code
}

fn unit_to_code(frozen_unit: &FrozenSchemaUnit) -> String {
    let mut code = String::new();

    use FrozenSchemaUnit::*;
    match frozen_unit {
        Namespace(_) => {}
        Import(_) => {}
        Constant { docstring, name, kind_value } => {
            let (kind_name, kind_value) = kind_value.name_and_value();
            code += &*format!("pub const {name}: {}", kind_name);

            if let Some(value) = kind_value {
                code += &*format!(" = {value}");
            }

            code += ";\n\n";
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
            // TODO: Only include async_trait if at least one of the functions needs it
            // code += "#[async_trait]\n";

            // Trait Start
            code += &*format!("pub trait {} {{\n", name);

            // Functions
            for function in functions {
                code += &*unit_to_code(function);
            }

            // Trait End
            code += "\n}\n\n";
        }
        Function {
            docstring, name, synchronous,
            arguments, _return,
            throws
        } => {
            code += "\t";

            // Async definition (with async_trait)
            if !synchronous { code += "async "; }

            // Function Start
            code += &*format!("fn {name}(");
            code += "&self";

            // Arguments
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
            code += ")";

            // Returns
            // code += format!("-> {}", return)

            // Function End
            code += ";\n";
        }
        Error { .. } => {}
        Validator { .. } => {}
        _ => panic!("Reached forbidden or unimplemented node")
    }

    code
}

/*
fn protocol_to_code(unit: FrozenUnit) -> String {
    let FrozenUnit::Protocol {
        docstring, parameters, name, functions
    } = unit else { panic!() };

    let mut code = String::new();

    // Trait Start
    // TODO: Maybe only include async_trait if at least one of the functions need it
    code.push_str("#[async_trait]");
    code.push_str(&*format!("pub trait {} {{", name));

    // Functions
    for function in functions {
        let FrozenUnit::Function {
            docstring, name, synchronous, direction,
            arguments, returns, throws
        } = function else { panic!() };

        let mut fn_code = String::new();

        // Async definition (with async_trait)
        if !synchronous {
            fn_code.push_str("async ");
        }

        // Fn
        fn_code.push_str(&*format!("fn {} (", name));

        // Arguments
        let mut idx = 0;
        while idx != arguments.len() {
            let FrozenUnit::Parameter {
                name, default_value
            } = arguments.get(idx).unwrap();

            if argument.id.is_none() {
                fn_code.push_str(&*format!("arg{}: {:?}", idx, argument.type_));
                if idx != arguments.len() { fn_code.push_str(", ") }
            } else {
                fn_code.push_str(
                    &*format!("{}: {:?}", argument.id.clone().unwrap(), argument.type_)
                );
                if idx != arguments.len() { fn_code.push_str(", ") }
            }

            idx += 1;
        }

        // Returns

        code.push_str(&*fn_code)
    }

    // Trait End
    code.push_str("}}");

    code.to_owned()
}
*/

