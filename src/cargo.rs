/// # Cargo
/// `cargo` is the official package management and build automation tool for
/// Rust. It includes dependency management integrated with the official Rust
/// package registry `crates.io`, unit testing, benchmarking, and examples. For
/// more automation look into `cargo-make`.
/// 
/// ## Project Initialization
/// You can set up a project with `cargo` using `cargo new my_executable` for a
/// binary, and `cargo new --lib my_library` for a library.
/// These will create a new folder containing the project files, in particular
/// the crate root `src/main.rs` (or `src/lib.rs`) and the build configuration
/// file `Cargo.toml`.
/// To build your project, run `cargo build`. To build and run it, `cargo run`.
/// 
/// ## Project Configuration
/// In the `Cargo.toml` there is the following information:
/// ```
/// [package]                               // meta-tags used by `crates.io`
/// name = "my_executable"                  // project name
/// version = "0.1.0"                       // semantic version
/// authors = ["jahrim"]                    // authors
/// 
/// [dependencies]                          // project dependencies
/// clap = "2.27.1"                         // from crates.io
/// rand = {                                // from git
///   git = "https://github.com/rust-lang-nursery/rand"
/// } 
/// my_library = { path = "../my_library" } // locally
/// ```
/// More information at https://doc.rust-lang.org/cargo/reference/manifest.html.
/// 
/// ## Build Scripts
/// Cargo allows you define a custom build script to be run before the project
/// is built. By default, this is the `build.rs` inside you project folder, but
/// you can change its location setting
/// ```
/// [package]
/// build = ".../my_build.rs
/// ```
/// 
/// ## Binaries
/// A project can contain multiple binaries, in addition to the cargo root.
/// These are located in the `src/bin` folder, for example
/// `src/bin/other_bin.rs`.
/// 
/// You can target a specific binary in the project using the `--bin` flag.
/// For example, you can run one using `cargo run --bin other_bin`.
fn cargo() {}