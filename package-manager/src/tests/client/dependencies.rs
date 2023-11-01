// Standard Uses

// Crate Uses
use crate::client::dependency;
use crate::tests::client::PACKAGE_CONTEXT;
use crate::tests::client::registries::PACKAGE_REGISTRIES_PATH;

// External Uses


#[test]
fn add_local_dependency() {
    let package_path = PACKAGE_REGISTRIES_PATH.join("test_package/");
    let dependency = dependency::get_local_dependency(&package_path).unwrap();

    let package_ctx = vec![PACKAGE_CONTEXT()];

    for (dep_version, expected_version) in dependency.iter().zip(package_ctx) {
        assert_eq!(&dep_version.0, expected_version.config_frozen.as_ref().unwrap());
        assert_eq!(
            expected_version.schema_contexts.iter()
                .map(|s| s.borrow().frozen_schema.clone().unwrap())
                .collect::<Vec<_>>(),

            dep_version.1
        );
    }
}

