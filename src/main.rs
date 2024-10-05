// --- PLAYGROUND SETUP --------------------------------------------------------
// Disable warnings at the crate level (must be on top of the crate root)
#![allow(warnings, unused)]

// Use other modules so that they are compiled
// Create modules for each file in the crate `src`, so they are compiled
#[macro_use] pub mod util;
mod annotations;
mod assignments;
mod cargo;
mod closures;
mod crates;
mod documentation;
mod enums;
mod errors;
mod expressions;
mod functions;
mod generics;
mod imports;
mod macros;
mod methods;
mod modules;
mod ownership;
mod pattern_matching;
mod primitives;
mod printing;
mod references;
mod structures;
mod unit_testing;
mod traits;
mod types;
mod unsafe_code;
// -----------------------------------------------------------------------------

/// # Entry Point (Main Function)
/// You can run the following program by:
/// - `cargo run` or `cargo run my_command_line_arguments`
/// - `rustc **/main.rs` and then execute the output binary `**/main`
/// - Clicking on `Run` in VsCode with Rust-Analyzer on top of a `main` function
fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean");

    /// ## Command Line Arguments
    /// The argument to the binary can be extracted from the environment.
    let args: Vec<String> = std::env::args().collect();
    let program: &String = &args[0];
    let program_args: &[String] = &args[1..];
    println!("Running {:?} with arguments {:?}", program, program_args);
}
