// Standard Uses

// Crate Uses

// External Uses
use toml_edit::{Document, table, value};


#[allow(unused)]
fn add_build_deps(config: &mut Document) -> &mut Document {
    config["build-dependencies"] = table();
    config["build-dependencies"]["cbindgen"] = value("0.24.0");

    config
}
