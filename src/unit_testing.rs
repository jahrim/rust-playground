/// # Unit Testing
/// In Rust, unit tests are defined in modules inside the source code. For
/// example, a module `implementation` would contain a submodule
/// `implementation::tests` containing its unit tests.
/// 
/// These test modules are usually marked with the annotation `#[cfg(test)]`,
/// which makes it so they are ignored during compilation when producing a
/// binary, making compilation faster.
/// 
/// Inside these module, every function marked with the annotation `#[test]` is
/// a unit test. A test succeeds by default, and fails if an unresolvable error
/// is thrown (i.e. `panic!`, `assert!`, `assert_eq!`, `assert_ne!`).
pub mod implementation {
    /// Definitions
    #[derive(Debug, PartialEq, Eq)] pub struct Num(usize);
    impl Num {
        pub fn add(&self, other: &Self) -> Self { Num(self.0 + other.0) }
        fn sub(&self, other: &Self) -> Self { Num(self.0 - other.0) }
        fn try_sub(&self, other: &Self) -> Result<Self, ()> { 
            self.0.checked_sub(other.0).map(|v| Num(v)).ok_or(())
        }
    }

    /// Unit Tests
    #[cfg(test)]  // Mark the following module as a container of unit tests
    mod tests {
        use super::*;  // Import everything you want to test
        // use crate::unit_testing::implementation::*;  // Same as above

        /// Testing Public Members
        #[test]   // Makes the following function an executable unit test
        fn test_add() {
            let x: Num = Num(5);
            let y: Num = Num(3);
            let result: Num = x.add(&y);
            let expectation: Num = Num(8);
            assert!(result.0 == expectation.0);
            assert_eq!(result, expectation);
            assert_ne!(result, x);
            assert_ne!(result, y);
        }

        /// ## Testing Private Members
        /// Differently from integration tests (located in the `tests` folder),
        /// unit test can access and test private members (this is because they
        /// are defined in a module inside the implementation).
        #[test]
        fn test_sub() {
            let x: Num = Num(5);
            let y: Num = Num(3);
            let result: Num = x.sub(&y);
            let expectation: Num = Num(2);
            assert!(result.0 == expectation.0);
            assert_eq!(result, expectation);
        }

        /// ## Testing Errors
        /// Tests can also check that errors are raised when expected.
        #[test]
        #[should_panic]
        fn test_sub_overflow() { Num(1).sub(&Num(3)); }
        #[test]
        #[should_panic(expected = "attempt to subtract with overflow")]
        fn test_sub_overflow_explicit() { Num(1).sub(&Num(3)); }

        /// ## Testing Results
        /// Latest versions of Rust allow to specify a test which succeeds when
        /// returning a Result.Ok<A>, and fails when returning a Result.Err<E>.
        /// This approach works very well with the `?` operator (see 
        /// `errors.rs`).
        #[test]
        fn test_add_with_result() -> Result<(), ()> {
            match Num(5).add(&Num(3)) {
                Num(8) => Ok(()),
                _ => Err(()),
            }
        }
        #[test]
        fn test_sub_chain_overflow() -> Result<(), ()> {
            Num(5)
                .try_sub(&Num(3))?  // If `Err`, then the test fails
                .try_sub(&Num(1))?
                .try_sub(&Num(5))
                .map_or_else(|err| Ok(()), |ok| Err(()))
        }

        /// ## Pending Tests
        /// You can annotate a test with `#[ignore]` to skip until it is fixed.
        #[test]
        #[ignore]
        fn ignore_until_fixed(){ todo!("fix me!!!") }
    }
}

/// # Documentation Tests
/// Often developers include code examples in the documentation of their APIs.
/// Rust treat this example as tests, namely Documentation Tests. These only
/// work on library crates.
mod documentation_tests {
    #[derive(Debug, PartialEq, Eq)] pub struct Num(usize);
    impl Num {
        /// Documentation Test
        /// ```
        /// let x: Num = Num(5);
        /// let y: Num = Num(3);
        /// let result: Num = x.add(&y);
        /// let expectation: Num = Num(8);
        /// assert!(result.0 == expectation.0);
        /// assert_eq!(result, expectation);
        /// assert_ne!(result, x);
        /// assert_ne!(result, y);
        /// ```
        /// 
        /// Disabled Documentation Test: this is part of the documentation, but
        /// it won't be run as a test.
        /// ```no_run
        /// let x: Num = Num(5);
        /// let y: Num = Num(3);
        /// let result: Num = x.add(&y);
        /// let expectation: Num = Num(8);
        /// assert!(result.0 == expectation.0);
        /// assert_eq!(result, expectation);
        /// assert_ne!(result, x);
        /// assert_ne!(result, y);
        /// ```
        pub fn add(&self, other: &Self) -> Self { Num(self.0 + other.0) }
    }
}

/// # Tests in Cargo
/// All unit, integration and documentation tests are run when with the command
/// `cargo test`. Flags and arguments are available for specifying which tests
/// to run.
fn tests_in_cargo() {}