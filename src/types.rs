use std::iter;

/// # Types
/// Rust is a statically typed languages. The compiler knows the type of any
/// value at any time.
runnable!(types, {
    /// ## Typed Assignments
    /// Every variable is bound to a type in Rust. You can either specify the
    /// type yourself or let the compiler infer it when possible.
    let decimal: f32 = 100.3;  // Explicit
    let decimal = 100.3;       // Inferred Unsuffixed
    let decimal = 100.3f32;    // Inferred Suffixed
    // let int: u8 = decimal;
    // ^ Error: type mismatch `u8` != `f32`

    /// ## Deferred Type Inference
    /// The Rust compiler can also look into the following instructions to
    /// infer the type of a variable.
    let mut vec = Vec::new();  // Vec is now assigned to the type Vec<{unknown}>
    println!("vec: {:?}", vec);
    vec.push(decimal);         // Vec is now assigned to the type Vec<f32>
    println!("vec: {:?}", vec);
});

/// ## Type Aliases
runnable!(type_aliases, {
    /// ## Type Aliasing
    /// Type aliases require UpperCamelCase names.
    type Coordinate = f64;
    type Point2D = (Coordinate, Coordinate);
    let point: Point2D = (1.03, 5.0);
    // let point: Point2D = Point2D(1.03, 5.0);
    // ^ Error: cannot use type-alias as a contructor
    println!("point: {:?}", point);
});

/// ## Casting
/// Casting is the operation of converting a variable from one type to another.
runnable!(casting, {
    /// ### Casting Primitives
    /// The `as` keyword can be used to safely cast between primitive types.
    let decimal: f32 = 20.0;
    let int: u8 = decimal as u8;                                // Safe
    unsafe { let int: u8 = decimal.to_int_unchecked::<u8>(); }  // Unsafe

    /// ### Custom Casting
    /// Rust provides generic conversion through inheritance of two traits:
    /// - `From.from(B) -> A`: create an instance of A *from* an instance of B
    /// - `Into.into(A) -> B`: map this instance of A *into* an instance of B
    /// 
    /// Implementing `From` gets the implementation of `Into` for free, but not
    /// viceversa.
    #[derive(Debug)] struct Number { underlying: i32 }
    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { underlying: item }
        }
    }
    impl Into<i32> for Number {
        fn into(self) -> i32 {
            self.underlying
        }
    }
    let number: Number = Number::from(10i32);
    println!("number: {}", number);
    let int: i32 = number.into();
    // ^ Here, `into` works only if the compiler can determine the output type
    // (`i32` in this case)    
    println!("int: {}", int);

    /// ### Custom Casting with Exceptions
    /// Similarly, there are two traits for casting with exceptions:
    /// - `TryFrom.try_from(B) -> Result(A, A::Error)`
    /// - `TryInto.try_into(A) -> Result(B, A::Error)`
    /// These require to specify the kind of `Error` expected when failing.
    impl TryFrom<char> for Number {
        type Error = ();  // The kind of `Error` is the unit type
        fn try_from(item: char) -> Result<Self, Self::Error> {
            let int: i32 = match item { 
                '0' => 0, '1' => 1, '2' => 2, '3' => 3, '4' => 4, '5' => 5,
                '6' => 6, '7' => 7, '8' => 8, '9' => 9, _otherwise => -1
            };
            if int == -1 { Err(()) } else { Ok(Number { underlying: int }) }
        }
    }
    let maybe_number1: Result<Number, ()> = Number::try_from('5');
    let maybe_number2: Result<Number, ()> = Number::try_from('a');
    let maybe_char1: Result<Number, ()> = '5'.try_into();
    let maybe_char2: Result<Number, ()> = 'a'.try_into();
    println!(
        "maybe_number1: {:?} maybe_number2: {:?} maybe_char1: {:?} maybe_char2: {:?}",
        maybe_number1, maybe_number2, maybe_char1, maybe_char2
    );

    /// ### Custom Casting to String
    /// Similarly, there are two traits:
    /// - `FromStr.parse::<A>(str) -> A`
    /// - `ToString.to_string(A) -> str`
    /// Instead of implementing `ToString` directly, it is better to implement
    /// `Display`, so you get both `println` and `to_string` at the same time.
    impl std::fmt::Display for Number {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Number with value {}", self.underlying)
        }
    }
    impl std::str::FromStr for Number {
        type Err = <i32 as std::str::FromStr>::Err;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let maybe_int: Result<i32, Self::Err> = s.parse();
            match maybe_int {
                Ok(int) => Ok(Number { underlying: int }),
                Err(e) => Err(e),
            }
        }
    }
    let number: Number = Number::from(10i32);
    println!("number: {}", number);
    let number_to_string: String = number.to_string();
    println!("number_to_string: {}", number_to_string);
    let string_to_number: Number = "5".parse().unwrap();
    println!("string_to_number: {}", string_to_number);
});