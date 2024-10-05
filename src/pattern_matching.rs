/// # Pattern Matching
/// Pattern matching in Rust is a combination of the `switch-case` semantics
/// with destructuring. Any `match` case must be totally-defined.
/// 
/// Note: you cannot pattern match on multiple types easily in Rust. This is
///       because there is no inheritance and no top type.
/// Note: std::any::Any is not a top type, but it's a wrapper used to simulate a
///       top type, supporting downcasting to any other type.
runnable!(pattern_matching, {
    enum Enum {
        Constant,
        TupleStruct(f32, f32),
        CStruct { x: f32, y: f32 },
    }
    struct CStruct { x: f32, y: f32 }

    /// ## Value Matching
    let value_matching = match 10 {
        1 => "Match a single value",
        2 | 4 | 7 => "Match multiple values",
        10..20 => "Match a range of values",
        int @ 30..40 => "Match a range of values, binding a name",
        /// ### Guards
        /// Cases can also match against conditions. Guards are not taken in
        /// consideration for checking that the domain of the match-case is
        /// totally-defined.
        int if int <= 0 && int > 40 => "Match zero",
        /// ### Default Case
        /// This last case matches against anything.
        otherwise => "Match anything else",
        _ => "Match anything else (never reached)",
    };
    println!("value_matching: {}", value_matching);

    /// ## Destructuring Matching
    let tuple_matching = 
        match (0, 1, 2) {
            (x, y, z) => "Match a triplet",
            (x, _, z) => "Match a triplet, ignoring the second element",
            (x, ..) => "Match any tuple, getting the first element",
            (.., z) => "Match any tuple, getting the last element",
            (x, .., z) => "Match any tuple, getting the first and last element",
        };
    println!("tuple_matching: {}", tuple_matching);

    let array_matching = 
        match [0, 1, 2] {
            [0, second, third] => "Match an array with 3 elements, whose head is 0",
            [head, tail @ ..] => "Match any non-empty array",
            [head, middle @.., last] => "Match any array with size > 2",
        };
    println!("array_matching: {}", array_matching);

    let enum_matching = 
        match Enum::Constant {
            Enum::Constant => "Match a constant",
            Enum::TupleStruct(x, y) => "Match a tuple struct",
            Enum::CStruct { x: a, y: b } => "Match a C struct",
        };
    println!("enum_matching: {}", enum_matching);
    
        
    let struct_matching = 
        match (CStruct { x: 0.0, y: 0.0 }) {
            CStruct { x: a, y: b } => "Match another C struct"
        };
    println!("struct_matching: {}", struct_matching);

    let reference_matching =
        match &0u8 {
            &value => "Match any reference, deconstructing its value",
        };
    println!("reference_matching: {}", reference_matching);
});

/// ## If-Let Pattern Matching for Conditional Expressions
runnable!(if_let, {
    let some: Option<u8> = Some(0);
    let none: Option<u8> = Some(0);

    // This is a bit awkward
    match some {
        Some(i) => println!("pattern_matching: {}", i),
        _ => (),
    }

    // This is the same
    if let Some(i) = some { println!("if_let: {}", i); }

    // You can also add more branches and pattern match on different variables
    if let Some(i) = some {
        println!("if_let_else_if: {}", i);
    } else if let Some(i) = none {
        println!("do something else")
    } else {
        println!("do something else")
    }
});

/// ## While-Let - Pattern Matching for Conditional Iterative Expressions
runnable!(while_let, {
    // This is a bit awkward
    let mut option: Option<u8> = Some(0);
    loop {
        match option {
            Some(i) => 
                if i == 10 {
                    option = None;
                } else {
                    option = Some(i + 1);
                }
            None => break,
        }
    }
    println!("loop_pattern_matching: {:?}", option);

    // This is the same
    let mut option: Option<u8> = Some(0);
    while let Some(i) = option {
        if i == 10 {
            option = None;
        } else {
            option = Some(i + 1);
        }
    }
    println!("while_let: {:?}", option);
});