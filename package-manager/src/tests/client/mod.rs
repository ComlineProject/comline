// Relative Modules
pub mod dependencies;
pub mod registries;
mod building;

// Standard Uses

// Crate Uses
use crate::client::publish;
use crate::tests::client::registries::{
    PACKAGE_REGISTRIES,
    registry_login_test, set_and_check_home_env_var, setup_local_registry
};

// External Uses
use comline_core::package::config::ir::context::{Origin, ProjectContext};
use comline_core::package::config::ir::frozen::FrozenUnit;


const COMLINE_HOME_NAME: &str = "COMLINE_HOME";
const COMLINE_HOME_PATH: &str = "~/.config/comline";


static PACKAGE_CONTEXT: fn() -> ProjectContext = || { ProjectContext {
    origin: Origin::Virtual,
    config: (Default::default(), vec![]),
    config_frozen: Some(vec![
        FrozenUnit::Namespace("test_package".to_owned()),
        FrozenUnit::PackageVersion("0.0.1".to_owned()),
        FrozenUnit::PublishRegistry(
            ("test".to_owned(), PACKAGE_REGISTRIES.get("test").cloned().unwrap())
        )
    ]),
    schema_contexts: vec![],
    relative_projects: vec![],
}};


#[test]
pub fn publish_into_test_registry() {
    set_and_check_home_env_var();
    setup_local_registry();

    registry_login_test();

    publish::publish_to_registry(
        &PACKAGE_CONTEXT(), "test".to_owned()
    ).unwrap();
}

/*
#[test]
fn publish_into_test_registries() {
    // publish::publish_to_registries(vec!["test".to_owned()]).unwrap();
    todo!()
}
*/

