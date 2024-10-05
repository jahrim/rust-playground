/// # Printing
/// Printing is handled by a series of `macros` defined in `std::fmt`:
/// - format! : can be used for string interpolation; returns a string
/// - print! : same as format!, but print to the stdout
/// - println! : same as print!, but appends a '\n'
/// - eprint! : same as print!, but print to the stderr
/// - eprintln! : same as println!, but print to the stderr
/// 
/// In these functions, placeholders are replaced by the following values, which
/// get stringified.
/// 
/// Placeholders differ depending on their purpose:
/// - `std::fmt::Display`: uses {} for user-friendly printing. `Display` is a
///    trait that can be implemented for converting types to pretty `String`s.
/// - `std::fmt::Debug`: uses {:?} for debugging purposes. `Debug` is a trait
///    that can be automatically derived for converting types to `String`s.
///    (similar to the default implementation of `toString()` in Java)
runnable!(printing, {
    // Ordered Arguments
    println!("{} days {} months", 31, 12);
    // Positional Arguments
    println!("{0} is less than {1}; {1} is greater than {0}", 10, 20);
    // Named Arguments
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Formatting
    println!("Base 10: \t{}", 69420);     // "69420"
    println!("Base 2 : \t{:b}", 69420);   // "10000111100101100"
    println!("Base 8 : \t{:o}", 69420);   // "207454"
    println!("Base 16: \t{:x}", 69420);   // "10f2c"
    
    println!("Justify right: \t|{number:>5}|", number=1);  // "    1"
    println!("Justify left:  \t|{number:<5}|", number=1);  // "1    "
    println!("Zero padding:  \t|{number:0>5}|", number=1); // "00001"
    println!("Zero padding:  \t|{number:0<5}|", number=1); // "10000"

    println!("Formatting with named arguments: {number:0<width$}",
             number=1,
             width=5);

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("Formatting with inferred arguments: {number:0<width$}");
});