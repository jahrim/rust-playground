// Here is a single-line comment
/* Here is a multi-line comment */

/// Here is a single-line doc-comment for the following item
fn following_item() {}

fn enclosing_item() {
    //! Here is a single-line doc-comment for the enclosing item
}

/// ## Documentation
/// **Documentation** follows _CommonMark Markdown specification_ and supports
/// `executable rust code samples` such as:
/// ```
/// # let y: u8 = 0u8; // hidden in the documentation; can be used in the sample
/// let x: u8 = y;     // this compiles, because `y` is defined, but hidden
/// ```
/// These samples will be compiled and executed as `Documentation Tests` if the
/// crate is a library (see unit_testing.rs).
fn documentation() {}

/// ## Documentation in Cargo
/// Use `cargo doc` to generate a static website in `target/doc`. You can also
/// open the documentation in your browser running `cargo doc --open`.
/// 
/// The website can be configured using annotations.
/// See https://doc.rust-lang.org/rust-by-example/meta/doc.html.
fn documentation_in_cargo(){}