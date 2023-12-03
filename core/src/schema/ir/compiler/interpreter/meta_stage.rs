// Standard Uses
use std::rc::Rc;
use std::cell::RefCell;

// Crate Uses
use crate::report::ReportDetails;
use crate::schema::ir::context::SchemaContext;
use crate::schema::ir::compiler::interpreter::semi_frozen;
use crate::schema::ir::compiler::report;
use crate::schema::ir::compiler::report::CompileError;
use crate::package::config::ir::context::ProjectContext;

// External Uses
use snafu::ResultExt;


pub fn compile_schema_metadata(
    schema_context: Rc<RefCell<SchemaContext>>,
    project_context: &ProjectContext
) -> Result<(), Box<dyn snafu::Error>> {
    let schema_ctx = schema_context.borrow();

    for i in 0..schema_ctx.schema.1.len() {
        let spanned_unit = &schema_ctx.schema.1[i];

        use crate::schema::idl::ast::unit::ASTUnit::*;
        match &spanned_unit.1 {
            Namespace(span, n) => {
                let name = n.clone();
                let Some(_) = project_context.find_schema_by_import(n) else {
                    Err(CompileError::NamespaceCollision {
                        origin: "aa".to_string(), target: "aaa".to_string()
                    }).context(report::CompileSnafu {
                        details: ReportDetails::fetch(&schema_ctx, span).unwrap()
                    })?
                };

                let mut compile_state = schema_ctx.compile_state.borrow_mut();
                if let Some(name) = &compile_state.namespace {
                    panic!("Namespace {} was already previously set", name)
                } else {
                    compile_state.namespace = Some(name)
                }
            }
            Docstring {..} => {
                todo!()
            }
            Import(span, import) => {
                let spanned = Rc::clone(spanned_unit);

                schema_ctx.compile_state.borrow_mut().imports.entry(spanned).or_insert(
                    semi_frozen::Import {
                        namespace: (*span, import.clone()),
                        // alias: alias.clone
                    }
                );
            }
            // Docstring { .. } => {}
            #[allow(unused)]
            Constant { name, ..} => {
                let spanned_unit = Rc::clone(spanned_unit);

                schema_ctx.compile_state.borrow_mut().consts
                    .entry(spanned_unit).or_insert(
                    semi_frozen::Constant { name: name.clone() }
                );
            }
            Struct { name, .. } => {
                let spanned_unit = Rc::clone(spanned_unit);

                schema_ctx.compile_state.borrow_mut().structures
                    .entry(spanned_unit).or_insert(
                    semi_frozen::Structure { name: name.clone() }
                );
            }
            Property { .. } => {}
            Parameter { .. } => {}
            ExpressionBlock { .. } => {}
            Enum { .. } => {}
            EnumVariant { .. } => {}
            Settings { .. } => {}
            Protocol { name, .. } => {
                let spanned_unit = Rc::clone(spanned_unit);

                schema_ctx.compile_state.borrow_mut().protocols
                    .entry(spanned_unit).or_insert(
                    semi_frozen::Protocol { name: name.clone() }
                );
            }
            Function { .. } => {}
            Argument { .. } => {}
            Error { .. } => {}
            Validator { .. } => {}
            Field { .. } => {}
        }
    }

    Ok(())
}