/// # Crate
/// Crates are the compilation unit in Rust. When the Rust Compiler `rustc` is
/// called onto a file, that file is treated as a crate, and specifically as the
/// `crate root`. Before compilation, any reference to other modules (`mod` and
/// `use`?) will be replaced by copying the content of the module where it is
/// referenced before compilation.
/// 
/// ## Crate Types
/// Crates can be compiled into either binaries or libraries, setting the flag
/// `crate-type`:
/// - Binaries: `rustc my_executable.rs` will output the executable file
///   `my_executable`
/// - Libraries: `rustc --crate-type=lib --crate-name=my_lib file.rs` will
///   output the file `my_lib.rlib`
/// 
/// ## Linking Libraries
/// To use a library inside a crate, you need to link it during compilation:
/// `rustc my_executable.rs --extern my_lib_module=my_lib.rlib`
/// The `extern` keyword will define a new module (here `my_lib_module`) at the
/// beginning of the `crate root`, containing all the definitions inside the
/// specified library (here `my_lib.rlib`).
fn crates() {}