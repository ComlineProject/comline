// Standard Uses
use std::path::Path;
use std::collections::HashMap;
use std::string::ToString;

// Crate Uses
use crate::client::publish;
use crate::tests::client::{COMLINE_HOME_NAME, COMLINE_HOME_PATH};

// External Uses
use comline_core::project::ir::frozen::{PublishRegistry, RegistryKind};

use once_cell::sync::Lazy;


pub static PACKAGE_REGISTRIES_PATH: Lazy<&Path> = Lazy::new(|| Path::new(
    "../__TEST_DATA__/.temp/registry/"
));

pub static PACKAGE_REGISTRIES: Lazy<HashMap<String, PublishRegistry>> = Lazy::new(||
    HashMap::from([
        ("test".to_owned(), PublishRegistry {
            kind: RegistryKind::LocalStorage,
            uri: PACKAGE_REGISTRIES_PATH.display().to_string()
        })
    ])
);

pub fn setup_local_registry() {
    let package_path = Path::new("../__TEST_DATA__/test/");
    let temp_path = package_path.parent().unwrap().join(".temp/");
    let registry_path = temp_path.join("registry/");

    if !registry_path.exists() {
        std::fs::create_dir_all(registry_path).unwrap();
    }
}

#[test]
pub fn set_and_check_home_env_var() {
    std::env::set_var(COMLINE_HOME_NAME, COMLINE_HOME_PATH);
    assert_eq!(std::env::var(COMLINE_HOME_NAME), Ok(COMLINE_HOME_PATH.to_string()));
}

#[test]
fn registry_login_default() {
    set_and_check_home_env_var();
    setup_local_registry();

    publish::login_to_registry(
        None, "default".to_owned(), "TOKEN".to_owned()
    ).unwrap();
}


#[test]
pub fn registry_login_test() {
    set_and_check_home_env_var();
    setup_local_registry();

    publish::login_to_registry(
        Some(&*PACKAGE_REGISTRIES),
        "test".to_owned(), "TEST_TOKEN".to_owned()
    ).unwrap();
}
