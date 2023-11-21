// Standard Uses

// Crate Uses

// External Uses
use comline::schema::ir::frozen::unit::FrozenUnit;


#[allow(unused)]
pub(crate) fn frozen_to_code(units: &Vec<FrozenUnit>) -> String {
    let mut code = "// Generated with Comline compiler and code generator\n\n"
        .to_owned();

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
            let (kind_name, kind_value) = kind_value.name_and_value();
            code += &*format!("const {name}: {}", kind_name);

            if let Some(value) = kind_value {
                code += &*format!(" = {value}");
            }

            code += ";\n";
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
        _ => panic!("Reached forbidden or unimplemented node")
    }

    code
}
