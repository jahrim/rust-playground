/// # Integration Tests
/// A project can contain several integration tests in the `tests` folder,
/// for example `tests/my_test.rs`.
///
/// To run all tests, you can use `cargo test`. To run all tests matching a
/// specific prefix, you can use `cargo test some_prefix`. All integration tests
/// (i.e. each file) are run concurrently. 
///
/// Each integration test is treated as a different crate, so it can only
/// access and test public members in the library. Note that integration tests
/// only make sense if your crate is a library, because their purpose it's to
/// test a public API from the perspective of the end-users.
///
/// In any other aspect, they are treated the same as unit tests (see
/// `unit_testing.rs`).

// use rust_template::unit_testing::implementation;
// ^ Error: `rust_template` is a binary crate, not a library crate

mod integration_test_module;    // define and import shared utilities

#[test]
fn integration_test(){
    integration_test_module::utility_function();

    // You could test `rust_template` here, if it was a library crate
    // ...
}