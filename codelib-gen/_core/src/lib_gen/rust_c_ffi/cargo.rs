// Standard Uses
use std::path::Path;

// Crate Uses
use crate::{code_gen, utils};

// External Uses
use comline_core::schema::ir::frozen::unit::{FrozenContextWhole as FrozenSchema};

use eyre::Result;
use toml_edit::{Document, Array, table, value};


#[allow(unused)]
pub fn generate_cargo_project(
    name: &str, version: &str, target: &Path, schemas: &Vec<FrozenSchema>
) -> Result<()> {
    let cargo_config_path = target.join("cargo.toml");
    std::fs::write(cargo_config_path, generate_cargo_config(name, version))?;

    let src_path = target.join("src/");
    std::fs::create_dir(src_path)?;

    for schema in schemas {
        let result = code_gen::rust::_1_7_0::frozen_schema_to_code(&schema.1);
    }


    todo!()
}

pub fn generate_cargo_config(name: &str, version: &str) -> String {
    let mut config = Document::new();

    config["package"] = table();
    config["package"]["name"] = value(name);
    config["package"]["version"] = value(version);
    config["package"]["edition"] = value("2021");


    config["lib"] = table();
    config["lib"]["name"] = value(name);

    let mut crate_type = Array::default();
    crate_type.push("dylib");
    config["lib"]["crate-type"] = value(crate_type);


    config["dependencies"] = table();
    config["dependencies"]["comline"]["path"] = value("../core");
    config["dependencies"]["comline-runtime"]["path"] = value("../runtime");
    config["dependencies"]["abi_stable"]["version"] = value("^0.11.2");

    let output = format!("{}\n\n{}", utils::generation_note("#"), config.to_string());

    output
}


mod tests {
    #[test]
    pub fn output() {
        let config = super::generate_cargo_config("test", "0.1");

        println!("{}", config);
    }
}

