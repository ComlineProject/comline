use std::path::PathBuf;

// Relative Modules
pub(crate) mod cargo;

// Standard Uses

// Crate Uses

// External Uses


#[allow(unused)]
fn build_bindings(crate_path: &PathBuf) {
    cbindgen::Builder::new()
        .with_crate(crate_path)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings.h");
}

