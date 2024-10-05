/// # Error Handling
/// Some functions only works under certain conditions, otherwise they could
/// throw an error. Rust offers different ways to handle these exceptions.
fn errors(){}

/// ## Unrecoverable Errors
/// Using the `panic!` macro, it is possible to throw unrecoverable errors,
/// terminating the program. This is very useful for unit testing and debugging,
/// but should be avoided otherwise.
runnable!(unrecoverable_errors, {
    panic!("I cannot recover from this!!!");

    /// Errors have the bottom type `!` and can be assigned to any typed
    /// variables during type-checking.
    let int: u8 = panic!("I am a u8");
    let string: String = panic!("I am a string");
    let branch: &str = if true { "correct!" } else { panic!("cannot recover") };

    /// The bottom type is experimental and cannot be used explicitly for typing
    /// variables. It can still be used to type functions.
    fn bottom() -> ! { panic!("I am the bottom type"); }
    // let bottom: ! = panic!("I am the bottom type");
    // ^ Error: type `!` is experimental
});

/// ## Unimplemented Errors
/// One particular use case of unrecoverable errors, it's to signal that a
/// function is yet to be implemented. In fact, there are also standard macros
/// for this specific use case.
runnable!(not_implemented, { 
    // `unimplemented` is usually interpreted as: "it may be done at some point"
    unimplemented!("Cannot implement yet");
    panic!("not implemented: Cannot implement yet");  // Same as above
});
runnable!(todo_later, {
    // `todo` is usually interpreted as: "it will be done at a later moment"
    todo!("I can implement this one, but I will later");
});

/// ## Panic Behaviors
/// The behavior of unrecoverable errors can be change in the configuration
/// when compiling the source code. The current behaviors are 'abort' and
/// 'unwind'. These can be combined with conditional compilation to support
/// unrecoverable and recoverable implementations of the same function.
#[cfg(panic = "abort")]
fn error<A>(recover: A) -> A { panic!("aborting...") }
#[cfg(panic = "unwind")]
fn error<A>(recover: A) -> A { eprintln!("recovering..."); recover }

fn sum_even_numbers(x: u8, y: u8) -> u8 {
    if x % 2 == 0 && y % 2 == 0 { x + y } else { error(0) }
}
runnable!(panic_behavior, {
    println!("result: {}", sum_even_numbers(0, 1));
});

/// ## Recoverable Errors
/// Error handling can be improved using types that are capable of representing
/// errors, such as `Option<A>` (either `Some<A>` or `None`) and `Result<A, E>`
/// (either `Ok<A>` or `Err<E>`). These allow the program to continue and the
/// API user to react to possible exceptions.
runnable!(options, {
    /// ### Options as Errors
    /// An `Option` is either a `Some` with a result or an empty `None`. `None`
    /// can be used to signal an exception without crashing the program.
    fn sum_even_numbers(x: u8, y: u8) -> Option<u8> {
        if x % 2 == 0 && y % 2 == 0 { Some(x + y) } else { None }
    }
    match sum_even_numbers(0, 1) {
        Some(result) => println!("Everything went well! Result:{}", result),
        None => println!("Something went wrong: inputs were not even")
    }

    /// ### Unwrapping Options
    /// There are many ways to get the content of an `Option`. These include:
    /// - `unwrap`: extract the value or panics with `no such element`
    /// - `expect`: extract the value of the `Option` or panics with a more
    ///             informative user-defined error.
    /// - `unwrap_or`: extract the value or return a default value.
    let a1: u8 = sum_even_numbers(0, 2).unwrap();
    let a2: u8 = sum_even_numbers(0, 2).expect("inputs were not even");
    let a3: u8 = sum_even_numbers(0, 11).unwrap_or(0);
    println!("a1: {} a2: {} a3: {}", a1, a2, a3);

    /// ### Concatenating Options
    /// The `?` operator can be used to extract the value of the `Option`, or
    /// return `None` from the enclosing scope.
    /// 
    /// Older versions of Rust, uses the `try!` macro, which works in the same
    /// way.
    trait SumEven { 
        fn sum(self, other: Self) -> Option<Self> where Self: Sized;
    }
    impl SumEven for u8 {
        fn sum(self, other: Self) -> Option<Self> {
            sum_even_numbers(self, other) 
        }
    }
    fn sum5_even(x: u8, y: u8, z: u8, v: u8, w: u8) -> Option<u8> {
        let option: Option<u8> = x.sum(y);
        let s: u8 = option?;
        // ^ Same as:
        // let s: u8 = 
        //     if option.is_some() { option.unwrap() }
        //     else { return None };
        s.sum(z)?.sum(v)?.sum(w)
    }
    println!("b1: {:?}", sum5_even(0, 2, 4, 11, 6));
});

runnable!(results, {
    /// ### Results as Errors
    /// A `Result` can be either an `Ok` with a "success" value or an `Err` with
    /// a "failure" value. `Err` can be used to signal more informative
    /// exceptions without crashing the program.
    /// 
    /// Note: `Result` is left-biased: Result<A, B> ~ Scala Either[B, A].
    fn sum_even_numbers(x: u8, y: u8) -> Result<u8, String> {
        if x % 2 == 0 && y % 2 == 0 {
            Ok(x + y)
        } else {
            Err(format!("illegal inputs: an input is not even: x={x} y={y}"))
        }
    }
    match sum_even_numbers(0, 1) {
        Ok(result) => println!("Everything went well! Result:{}", result),
        Err(msg) => println!("Something went wrong: {}", msg)
    }

    /// ### Unwrapping Results
    /// Unwrapping a result works similarly to `Option`s.
    let a1: u8 = sum_even_numbers(0, 2).unwrap();
    let a2: u8 = sum_even_numbers(0, 2).expect("1 is not even");
    let a3: u8 = sum_even_numbers(0, 11).unwrap_or(0);
    println!("a1: {} a2: {} a3: {}", a1, a2, a3);

    /// ### Concatenating Results
    /// Concatening `Result`s works similarly to `Option`s.
    trait SumEven { 
        fn sum(self, other: Self) -> Result<Self, String> where Self: Sized;
    }
    impl SumEven for u8 {
        fn sum(self, other: Self) -> Result<Self, String> {
            sum_even_numbers(self, other) 
        }
    }
    fn sum5_even(x: u8, y: u8, z: u8, v: u8, w: u8) -> Result<u8, String>{
        let result: Result<u8, String> = x.sum(y);
        let s: u8 = result?;
        // ^ Same as:
        // let s: u8 = 
        //     if result.is_ok() { result.unwrap() }
        //     else { return Err(result.unwrap_err()) };
        s.sum(z)?.sum(v)?.sum(w)
    }
    println!("b1: {:?}", sum5_even(0, 2, 4, 11, 6));
});

/// ## Error Hierachies
/// Rust has no subtyping, so creating error hierarchies is not possible. This
/// means that handling different types of errors is a bit more convoluted.
/// 
/// A common way to do that is to define your own error types, implementing
/// the `std::error::Error` built-in trait object. Then, any error can be
/// returned in a `Box<dyn Error>`, abstracting the concrete type of the
/// instance.
/// 
/// This approach can be combined with enum types to reduce boilerplate and
/// enable pattern matching on errors.
#[derive(Debug)]
enum VectorError {
    EmptyVector,
    NotFound(usize),
}

// Implement `Display` to encode error messages
impl std::fmt::Display for VectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            VectorError::EmptyVector =>
                write!(f, "expected non-empty vector"),
            VectorError::NotFound(index) =>
                write!(f, "element at index {} not found", index),
        }
    }
}

// Implement `std::error::Error`, which is the top type for errors
impl std::error::Error for VectorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Return the cause of this `std::error::Error`, if any
        match *self {
            VectorError::EmptyVector => None,
            VectorError::NotFound(index) => None,
        }
    }
}

// These two functions throw different errors. However, the API abstracts from
// their concrete types, so the `Result`s can be more easily concatenated.
fn head<A>(array: &[A]) -> Result<&A, VectorError> {
    if array.len() > 0 { Ok(&array[0]) } else { Err(VectorError::EmptyVector) }
}
fn get<A>(array: &[A], index: usize) -> Result<&A, VectorError> {
    let mut current: usize = 0;
    let length: usize = array.len();
    for x in array.iter(){
        if current == index {
            return Ok(x)
        } else {
            current += 1;
        }
    }
    Err(VectorError::NotFound(index))
}

// If your errors implement `std::error::Error`
runnable!(error_hierachies, {
    use std::error::Error;

    // Upcasting
    let mut any_error: Box<dyn Error> = Box::new("a".parse::<u8>().unwrap_err());
    println!("any_error: {:?}", any_error);

    any_error = Box::new(VectorError::NotFound(1));
    println!("any_error: {:?}", any_error);

    any_error = Box::new(VectorError::EmptyVector);
    println!("any_error: {:?}", any_error);

    // Downcasting
    let vector_error_result = any_error.downcast::<VectorError>();
    println!("vector_error_result: {:?}", vector_error_result);

    match vector_error_result {
        Ok(vector_error) =>
            match *vector_error {
                VectorError::EmptyVector => println!("empty vector!"),
                VectorError::NotFound(i) => println!("not found {}", i),
            },
        Err(downcast_error) =>
            println!("downcasting failed with {}", downcast_error),
    }
});