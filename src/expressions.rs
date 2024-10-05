/// # Expressions
/// In Rust, every expression returns a value (like functional programming).
/// Statements (i.e. anything ending in `;`) return the `unit` value `()`.
const ONE: i32 = {
    let zero: i32 = 0;
    zero + 1
};

const UNIT: () = {
    let var: char = 'a';
};

/// ## Conditional Expressions
/// Conditional expressions can be defined using the `if-else` structure. All
/// branches must return the same type.
runnable!(if_else, {
    let result =
        if 5 > 0 && 5 < 0 || !(5 == 0) {
            1
        } else {
            0
        };
    println!("result: {}", result);
});

/// ## Iterative Expressions
/// Iterative expressions can be defined using the `loop` keyword. These are
/// infinite loops with go-to semantics, returning a value as any other
/// expression.
runnable!(infinite_loop, {
    let mut i: u8 = 0;
    let result = 
        loop {                  // Loop indefinitely
            println!("{}", i);
            i += 1;
            if i % 2 == 0 {
                continue;       // Skip to next cycle
            }
            if i == 99 {
                break;          // Exit from loop, returning unit
            }
        };
    println!("result: {:?}", result);
});

runnable!(goto_nested_loop, {
    'outer: loop {                  // Labelled loop
        println!("outer");
        'inner: loop {
            println!("inner");
            'innerinner: loop {
                println!("innerinner");
                break;              // Exit from 'innerinner
            }
            break 'outer;           // Exit from 'outer
        }
    }
});

runnable!(yield_loop, {
    let result: i32 = {
        let mut count: i32 = 0;
        loop {
            count += 1;
            if count == 100 { 
                break count;        // Exit from loop, returning `count`
            }
        }
    };
    println!("result: {}", result);
});

/// ## Conditionally Iterative Expressions
/// A conditionally iterative expression can be defined using the `while`
/// keyword.
runnable!(while_loop, {
    let mut count = 1;
    while count < 100 {
        count += 1
    }
    println!("count: {}", count);
});

/// ## Traverse Iterators
/// The `for-in` construct can be used to traverse any iterator.
runnable!(for_each_in_range, {
    for n in 1..101 { println!("{}", n); }      // Exclusive Range
    for n in 1..=100 { println!("{}", n); }     // Inclusive Range
});

runnable!(for_each_in_iterator, {
    let mut names = vec!["Bob", "Frank", "Harris"];
    
    // Borrows each element each iteration.
    // `names` cannot change in the loop              
    for name in names.iter() { println!("{}", name); }
    // Mutably borrows each element each iteration.
    // `names` may change in the loop          
    for name in names.iter_mut() { println!("{}", name); }
    // Consume the collection during the iteration (better memory management).
    // `names` is undefined after the loop     
    for name in names.into_iter() { println!("{}", name);  }
    // Same as into_iter()
    // Note: names must be redefined because it was previously consumed
    let mut names = vec!["Bob", "Frank", "Harris"];
    for name in names { println!("{}", name);  }
});