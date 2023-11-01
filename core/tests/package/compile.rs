// Standard Uses

// Crate Uses
use crate::package::TEST_PACKAGE_CFG_PATH;

// External Uses
use comline::project::ir::compiler::Compile;
use comline::project::ir::interpreter::ProjectInterpreter;


#[allow(unused)]
#[test]
fn compile_package() {
    let compiled = ProjectInterpreter::from_origin(&TEST_PACKAGE_CFG_PATH)
        .unwrap();

    todo!()
    // pretty_assertions::assert_eq!(compiled, ());
}
