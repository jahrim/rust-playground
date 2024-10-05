/// # Closures (~ Scala Lambdas)
/// Closures are like functions, but they are defined in a local scope. They can
/// use *captured values* from their scope, in addition to their inputs.
/// 
/// In Rust, there are different types of closures depending on how the captured
/// values are used by the closure.

/// ## Immutable Closures
/// Immutable closures have type `Fn(input_types...) -> output_type`.
/// Note: `impl X` means "anonymous struct implementing X"
runnable!(immutable_closures, {
    // one-line 0-ary lambda
    let one = || 1i32;       
    // one-line unary lambda             
    let add_one = |x: i32| x + one();       
    // multi-line binary lambda        
    let sum = |x: i32, y: i32| {                    
        let result: i32 = x + y;
        println!("{} + {} = {}", x, y, result);
        result
    };
    sum(3, 5);
});

/// ## Mutable Closures
/// Mutable closures have type `FnMut(input_types...) -> output_type`.
/// They modify some captured value.
runnable!(mutable_closures, {
    let mut count = 0;
    
    let mut inc = || {         // `mut` is required
        count = count + 1;
        println!("{}", count)
    };
    inc();
    inc();
    // let c: &i32 = &count;
    // ^ Error: cannot borrow `count`: already mutably borrowed by `inc`
    inc();
});

/// ## Consuming Closure
/// Consuming lambdas have type `FnOnce(input_types...) -> output_type`
/// They consume some captured value, so they can only be called once.
runnable!(consuming_lambda, {
    use std::mem;
    let int: Box<i32>  = Box::new(0);
    let consume = || {
        println!("{} consumed", int);
        mem::drop(int);
    };
    consume();
    // consume();
    // ^ Error: `int` has been freed and cannot be accessed anymore
});

/// ## Higher-Order Functions
/// When a closure is created, the compiler creates a new anonymous `struct`
/// implementing the safest typeclass (in order, `Fn`, `FnMut`, `FnOnce`)
/// suitable for their body.
/// 
/// To capture all the possible anonymous `struct`s, an higher-order function
/// requires to be generic in the type of the lambda to be applied. 
/// 
/// Constraints can be applied by requiring the generic type to implement
/// specific traits.
runnable!(higher_order_functions, {
    // `Lambda` can be anything (so, any anonymous `struct`)
    fn apply_lambda0<Lambda, O>(f: Lambda) -> O 
    // that implements `apply() -> O`
    where Lambda: Fn() -> O {                  
        f()
    }

    // Shorter version
    fn _apply_lambda0<Lambda: Fn() -> O, O>(f: Lambda) -> O { f() }

    // Even shorter: `f` is an anonymous `struct` of `apply() -> O`
    fn __apply_lambda0<O>(f: impl Fn() -> O) -> O { f() }

    fn apply_lambda1<Lambda, I, O>(f: Lambda, arg0: I) -> O
    where Lambda: Fn(I) -> O {
        f(arg0)
    }

    fn apply_lambda2<Lambda, I1, I2, O>(f: Lambda, arg0: I1, arg1: I2) -> O
    where Lambda: Fn(I1, I2) -> O {
        f(arg0, arg1)
    }

    // Lambdas can be used as input parameters
    println!("{}", apply_lambda0(|| 100));
    println!("{}", apply_lambda1(|x| x * 10, 10));
    println!("{}", apply_lambda2(|x, y| x + y, 40, 60));

    // Functions can also be used as input parameters
    fn one_hundred() -> i32 { 100 }
    println!("{}", apply_lambda0(one_hundred));
});

/// ## Function Composition
/// Since the type of a lambda is anonymous, similar rules apply for returning
/// them as an output.
runnable!(function_composition, {
    fn new_supplier<A: Copy>(x: A) -> impl Fn() -> A {
        // `move` forces all variables to be captured by value and not by
        // reference, otherwise they would be destroyed at the end of the
        // function. This require captured variables to implement the `Copy`
        // trait
        move || x                  
    }

    fn compose<F: Copy, G: Copy, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
    where F: Fn(A) -> B, G: Fn(B) -> C {
        move |x: A| g(f(x))
    }

    let mut x = 0i32;
    let supplier = new_supplier(x);
    println!("x={} supplier={}", x, supplier());
    x += 1;
    println!("x={} supplier={}", x, supplier());

    let add_one = |x| x + 1;
    let add_two = compose(add_one, add_one);
    println!("x={} x+2={}", x, add_two(x));
});
