/// # Functions
/// Functions can be defined using the `fn` keyword. Function names are ASCII,
/// alphanumeric, possibly containing '_', starting with either a letter or '_'.
fn sample_function(arg: u32) -> u32 {
    // you can forward-reference other functions
    let result: u32 = increment(arg);   
    // the last expression is the output of the function  
    result                              
}

fn increment(arg: u32) -> u32 {
    // you can nest function definitions and use them locally
    fn one() -> u32 { 
        return 1;  // you can explicitly return using the `return` keyword
        return 2;  // this will never be executed
    }
    arg + one() 
}

// Function with no real output, always return `unit` as output
fn unit_return_type1() -> () { println!("Done!") }

// The `unit` return type can be omitted
fn unit_return_type2() { println!("Done!") }

/// ## Raw Identifiers
/// Functions from old Rust versions may be named after some recently reserved
/// keyword. You can call them by prefixing their names with `r#`.
runnable!(raw_identifiers, {
    let r#fn = increment;       // `fn` is a keyword
    println!("{}", r#fn(0));
});