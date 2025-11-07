#![doc = include_str!("../README.md")]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

use melior::ir::Module;

/// Asserts that the given module is equal to the given string representation.
///
/// The string representation is roundtripped with the parser to normalize it. This makes the tests
/// less prone to flakiness because of things like whitespacing or value names.
///
/// # Example
///
/// ```
/// use melior::{
///    Context,
///    dialect::{DialectRegistry, arith, func},
///    ir::{
///     attribute::{StringAttribute, TypeAttribute},
///  operation::OperationLike,
///        r#type::FunctionType,
///     *,
///    },
/// utility::register_all_dialects,
/// };
///
///let registry = DialectRegistry::new();
///register_all_dialects(&registry);
///
///let context = Context::new();
///context.append_dialect_registry(&registry);
///context.load_all_available_dialects();
///
///let location = Location::unknown(&context);
///let module = Module::new(location);
///
///let index_type = Type::index(&context);
///
///module.body().append_operation(func::func(
///    &context,
/// StringAttribute::new(&context, "add"),
///    TypeAttribute::new(
///     FunctionType::new(&context, &[index_type, index_type], &[index_type]).into(),
///    ),
/// {
///  let block = Block::new(&[(index_type, location), (index_type, location)]);
///
///     let sum = block
///      .append_operation(arith::addi(
///       block.argument(0).unwrap().into(),
///    block.argument(1).unwrap().into(),
/// location,
///            ))
///         .result(0)
///      .unwrap();
///
///        block.append_operation(func::r#return(&[sum.into()], location));
///
///        let region = Region::new();
///        region.append_block(block);
///        region
///    },
///    &[],
///    location,
///));
///
/// // Note how this IR is not canonical wrt SSA names and formatting.
/// let repr = r#"
/// module {
///     func.func @add(%x: index, %y: index) -> index {
///         %r = arith.addi %x, %y: index
///         func.return %r : index
///     }
/// }
/// "#;
///
///mlir_testutils::assert_module_eq(&module, repr);
/// ```
pub fn assert_module_eq(module: &Module, repr: &str) {
    let module_str = format!("{}", module.as_operation());
    let ctx = module.context();
    let repr_mod = Module::parse(unsafe { ctx.to_ref() }, repr)
        .unwrap_or_else(|| panic!("Failed to parse check: {repr}"));
    let repr_str = format!("{}", repr_mod.as_operation());

    #[cfg(feature = "similar-asserts")]
    similar_asserts::assert_eq!(module_str, repr_str);
    #[cfg(not(feature = "similar-asserts"))]
    assert_eq!(module_str, repr_str);
}

/// Asserts that the given module is equal to the contents of a file in the given path.
///
/// The file content is roundtripped with the parser to normalize it. This makes the tests
/// less prone to flakiness because of things like whitespacing or value names.
///
/// # Example
///
/// ```
/// use melior::{
///    Context,
///    dialect::{DialectRegistry, arith, func},
///    ir::{
///     attribute::{StringAttribute, TypeAttribute},
///  operation::OperationLike,
///        r#type::FunctionType,
///     *,
///    },
/// utility::register_all_dialects,
/// };
///
///let registry = DialectRegistry::new();
///register_all_dialects(&registry);
///
///let context = Context::new();
///context.append_dialect_registry(&registry);
///context.load_all_available_dialects();
///
///let location = Location::unknown(&context);
///let module = Module::new(location);
///
///let index_type = Type::index(&context);
///
///module.body().append_operation(func::func(
///    &context,
/// StringAttribute::new(&context, "add"),
///    TypeAttribute::new(
///     FunctionType::new(&context, &[index_type, index_type], &[index_type]).into(),
///    ),
/// {
///  let block = Block::new(&[(index_type, location), (index_type, location)]);
///
///     let sum = block
///      .append_operation(arith::addi(
///       block.argument(0).unwrap().into(),
///    block.argument(1).unwrap().into(),
/// location,
///            ))
///         .result(0)
///      .unwrap();
///
///        block.append_operation(func::r#return(&[sum.into()], location));
///
///        let region = Region::new();
///        region.append_block(block);
///        region
///    },
///    &[],
///    location,
///));
///
///
///mlir_testutils::assert_module_eq_to_file!(&module, "sample.mlir");
/// ```

#[macro_export]
macro_rules! assert_module_eq_to_file {
    ($module:expr, $file:literal) => {
        $crate::assert_module_eq($module, include_str!($file));
    };
}
