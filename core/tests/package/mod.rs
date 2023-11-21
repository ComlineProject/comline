// Relative Modules
mod parse;
mod compile;
mod build;

// Standard Uses
use std::path::{Path, PathBuf};

// Crate Uses

// External Uses
use once_cell::sync::Lazy;
use comline::project::idl::constants::CONGREGATION_EXTENSION;


static TEST_PACKAGE_DIR: Lazy<&Path> = Lazy::new(|| Path::new("../__TEST_DATA__/test/"));
static TEST_PACKAGE_CFG_PATH: Lazy<PathBuf> = Lazy::new(||
    TEST_PACKAGE_DIR.join(format!("config.{}", CONGREGATION_EXTENSION))
);


/*
#[test]
fn parse_package_project() {
    let project = project::idl::parser::from_path(&TEST_PACKAGE_CFG_PATH).unwrap();

    pretty_assertions::assert_eq!(
        project,
        (
            VIndex {
                meta: UnitIndex::Index { path: "".to_string(), source: "".to_string() },
                nodes: vec![]
            },
            vec![
                ASTUnit::Congregation("".to_owned()),
                ASTUnit::Dependencies(vec![

                ])
            ]
        )
    )
}
*/
