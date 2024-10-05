/// # Attributes (~ Java Annotations)
/// Annotations can be used to add some metadata to the code structure (modules,
/// crate or individual items). They have several use cases.
/// 
/// ## Conditional Compilation
/// Conditional compilation allows the developer to decide which code should be
/// included in the binary depending on the values of certain compilation flags.
/// ```
/// #[cfg(target_os = "linux")]
/// fn only_compiled_for_linux_os() {}
/// #[cfg(not(target_os = "linux"))]
/// fn only_compiled_for_non_linux_os() {}
/// 
/// fn main(){
///     only_compiled_for_linux_os();  // this is removed on non-linux os
///     println!("Is this Linux? {}", cfg!(target_os = "linux"));
/// }
///  
/// #[cfg(my_custom_condition)]
/// fn only_compiled_if_my_custom_condition_holds() {
///     // `my_custom_condition` can be set to true by running
///     // `rustc --cfg my_custom_condition file.rs`
/// }
/// ```
/// 
/// ## Crate Configuration
/// Certain crate attributes can be configured through annotations. This only
/// works for `rustc`, as `cargo` will ignore these annotations.
/// ```
/// #![crate_type = "lib"]
/// #![crate_name = "my_lib"]
/// ```
/// 
/// ## Compiler Configuration
/// The compiler can also be configured through annotations, generally
/// enabling/disabling some of its features.
/// 
/// ```
/// // disable checks for all warnings and unused definitions
/// #![allow(warnings, unused)]  
/// ```
/// 
/// ```
/// // avoid linking the std library
/// #![no_std]  
/// ```
/// 
/// ## Linking Foreign Libraries
/// Annotations are used to import foreign libraries, which are functions
/// defined in languages other than Rust (see unsafe_code.rs).
/// 
/// ## Testing and Benchmarking
/// Annotations are used to define unit/integration tests and benchmarks (see
/// unit_tests.rs and implementation_tests.rs).
/// 
/// ## Macros
/// Annotations are used to export and import macros from libraries (see
/// macros.rs).
fn annotations() {}

/// Annotation for the Following Item
#[allow(warnings /* Attribute Argument */, unused)]
fn following_item() {}

/// Annotation for the Enclosing Item
fn enclosing_item() { #![allow(warning, unused)] }