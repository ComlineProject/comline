// Compiled Languages
pub(crate) mod rust;
pub(crate) mod rust_c_ffi;
pub(crate) mod c_lang;

// Dynamic Languages
pub(crate) mod lua;
pub(crate) mod luau;
pub(crate) mod python;

// Standard Uses
use std::collections::HashMap;

// Crate Uses

// External Uses
use comline::schema::ir::frozen::unit::FrozenUnit;
use once_cell::sync::Lazy;


#[allow(unused)]
pub type VersionGenerators = Lazy<HashMap<&'static str, GeneratorFn>>;
#[allow(unused)]
pub type GeneratorFn = fn(&Vec<FrozenUnit>) -> String;
#[allow(unused)]
pub type Generator = (GeneratorFn, &'static str);

#[allow(unused)]
static LANG_GENERATORS: Lazy<HashMap<&str, (&VersionGenerators, &str)>> = Lazy::new(|| {
    HashMap::from([
        ("rust", (&rust::GENERATORS, "rs")),

        ("luau", (&luau::GENERATORS, "luau")),
        ("python", (&python::GENERATORS, "py"))
    ])
});

#[allow(unused)]
pub fn find_generator(name: &str, version: &str) -> Option<(&'static GeneratorFn, &'static str)> {
    if let Some((lang_generator, extension)) = LANG_GENERATORS.get(name) {
        if let Some(version_generator) = lang_generator.get(version) {
            return Some((version_generator, extension))
        }
    };

    None
}