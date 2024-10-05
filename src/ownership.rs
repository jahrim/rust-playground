/// # Ownership
/// Variables in Rust do not only hold data in the stack, but they also own
/// `resources` (e.g., Box<T> owns some memory in the heap). Any variable is
/// responsible to free its own resources.
/// 
/// To avoid resources to be freed more than once, any resource can only have
/// one owner at a given time.
fn ownership() {}

/// ## RAII (Resource Acquisition Is Initialization)
/// Rust enforces RAII: when a structure is initialized in a scope, it will
/// start owning some resources; when a structure leaves its initialization
/// scope, its `destructor` is called and its resources freed. A `destructor` is
/// automatically implemented for any type, but you can write you own by
/// implementing the trait `Drop`.
///
/// RAII avoids resource leak bugs, such as memory leaks (you can monitor memory 
/// leaks using `valgrind`: `rustc main.rs && valgrind ./main`).
/// 
/// Thanks to the ownership mechanism, there no need to manage memory 
/// manually. Resources are deallocated automatically.
fn raii(){
    let structure: Box<u8> = Box::new(0);  // Start of ownership
    // <-- Here, the variable `structure` owns its location in the heap
    return;
    // After exiting its initialization scope, `structure` is destroyed
    // (i.e., its location in the heap is released for future variables)
}

runnable!(automatic_free, {
    for _ in 0..1_000_000 {
        Box::new(0u8);          // automatically freed
    }
});

/// ## Moving
/// If initializing a variable with a value *creates* ownership, initializing
/// a variable with another variable *transfer* ownership between variables.
/// This process is calles `move` of ownership.
/// Note: initialization also happens calling a function, as the function
///       parameters are initialized to the function arguments.
runnable!(stack_allocation_implies_copying, {
    let x: u8 = 0;  // stack allocation    
    let y: u8 = x;  // `x` is *copied* into `y`: two ownerships           
    // <-- Here both `x` and `y` exists
    println!("x: {}", x);
    println!("y: {}", y);

    // stack variables are passed by value, so ownership is retained
    fn use_var(x: u8) {}
    use_var(x);  
    use_var(y);
    // <-- Here both `x` and `y` exists
    println!("x: {}", x);
    println!("y: {}", y);
});

runnable!(heap_allocation_implies_moving, {
    
    let x: Box<u8> = Box::new(0);  // heap allocation
    let y: Box<u8> = x;  // `x`'s ownership is *moved* into `y`: one ownership
    // <-- Here the variable `x` has been deleted, only `y` retains ownership    
    // println!("{}", x); 
    // ^ Error: use of `x` after move
    println!("y: {}", y);    

    // heap variables are passed by reference, so ownership is transferred
    fn use_var(a: Box<u8>) { println!("a: {}", a); }
    // use_var(x); 
    // ^ Error: use of `x` after move
    use_var(y);
    // <-- Here `y` has been destroyed (ownership was given to `a` in `use_var`)
    // println!("{}", y);
    // ^ Error: use of `y` after move
});

/// Mutability of data can be changed during a `move`. This is possible because
/// the previous variable cannot be used anymore anyway.
fn mutability(){
    let immutable: Box<u8> = Box::new(0);
    // *immutable = 1;
    // ^ Error: cannot change immutable variable

    let mut mutable: Box<u8> = immutable;
    *mutable = 1;

    let immutable: Box<u8> = mutable;
    // *immutable = 2;
    // ^ Error: cannot change immutable variable
}

/// ## Partial Moving
/// When dealing with structures, it is possible to transfer the ownership of
/// only parts of the structure. The structure as a whole cannot be accessed
/// anymore, but you main retain ownership of some of its parts.
runnable!(partial_move, {
    #[derive(Debug)]
    struct Person { name: String, age: Box<u8> }

    let paul: Person = Person { 
        name: String::from("Paul"), 
        age: Box::new(20),
    };

    let Person {
        name,     // ownership of `paul.name` is moved to `name`
        ref age,  // ownership of `paul.age` is not moved to `age`
    } = paul;
    
    // println!("{:?}", paul);
    // ^ Error: use of `paul` after partial move
    println!("name: {}", name);
    // println!("paul.name: {}", paul.name);
    // ^ Error: use of `paul.name` after move
    println!("age: {}", age);
    println!("paul.age: {}", paul.age);
});

/// ## Borrowing
/// Most of the time we would like to access data without triggering a `move` of
/// ownership. To do so, you can pass data by reference `&T`, instead of by
/// value `T`. This is the `borrow` mechanism.
/// 
/// The `Borrow Checker` guarantees that all references always point to valid
/// structures in memory (i.e., the object cannot be destroyed if it is
/// referenced by a pointer).

fn delete<A>(x: A){ /* takes ownership */ }
fn borrow<A>(x: &A){ /* does not take ownership */ }
fn borrow_mut<A>(x: &mut A){ /* does not take ownership */ }

runnable!(borrowing, {
    let heap = Box::new(0u8);
    let stack = &0u8;

    borrow(&heap);
    borrow(&stack);
    // <-- Ownerships of `heap` and `stack` are retained
    borrow(&heap);
    borrow(&stack);

    delete(heap);
    delete(stack);
    // <-- Ownership of `heap` is lost; `stack` is retained (passed by value)
    // borrow(&heap);
    // ^ Error: use of `heap` after move
    borrow(&stack);
});

/// The borrow checker will give compile-time errors ensuring correct use of the
/// borrowing mechanism.
runnable!(borrow_checker, {
    let heap = Box::new(0u8);
    let borrowed: &u8 = &heap;
    // <-- Ownership of `heap` is retained; `heap` is borrowed
    //     (it is used later by `borrowed`)
    
    borrow(&heap);
    // delete(heap) 
    // ^ Error: `heap` is still borrowed (it is used later by `borrowed`)

    let x = *borrowed;
    // <-- Ownership of `heap` is retained; `heap` is no longer borrowed
    //     (no further use of `borrowed`)

    delete(heap);
    // <-- Ownership of `heap` is lost
    // borrow(heap);  
    // ^ Error: use of `heap` after move
});

/// Mutability and destructuring have some interactions with the ownership
/// mechanism.
runnable!(borrow_and_mutability, {
    let heap = Box::new(0u8);
    let mut heap_mut = Box::new(0u8);

    // Immutable references can be immutably borrowed
    borrow(&heap);
    // Mutable references can be immutably borrowed
    borrow(&heap_mut); 
    // Immutable references cannot be mutably borrowed
    // borrow_mut(&mut heap);  // Error
    // Mutable references can be mutably borrowed
    borrow_mut(&mut heap_mut); 

    // There can be many immutable borrows at the same time
    let borrowed1 = &heap;
    let borrowed2 = &heap_mut;
    let x = **borrowed1 + **borrowed2;
    delete(heap);

    // There can be only be a mutable borrow at a time
    let borrowed_mut1 = &mut heap_mut;
    // let borrowed_mut2 = &mut heap_mut; // Error
    // let x = **borrowed_mut1 + **borrowed_mut2; // Error

    // You cannot mix mutable and immutable borrows
    let borrowed1 = &heap_mut;
    // let borrowed_mut2 = &mut heap_mut; // Error
    // let x = **borrowed1 + **borrowed_mut2; // Error
    delete(heap_mut);
});

runnable!(borrow_and_destructuring, {
    let mut heap_mut = Box::new(0u8);
    println!("match: {}", match heap_mut {
        ref immutable_borrow =>
            "Match any variable, deconstructing an immutable borrow",
        ref mut mutable_borrow =>
            "Match any variable, deconstructing a mutable borrow",
    });
});

/// ## Lifetimes
/// Lifetimes are type annotations used by the borrow checker to verify that the
/// borrowing mechanism is used correctly. They annotate a reference `&`, so
/// that the borrow checker can keep track of when a variable is initialized and
/// when it can be destroyed.
fn lifetimes() {}

/// ### Scope Lifetime
/// Many times, a lifetime coincide with the scope of a variable.
runnable!(scope_lifetime, {
    let i = 3;                     // Lifetime of `i` starts
    {
        let borrow1 = &i;          // Lifetime of `borrow1` starts
    }                              // Lifetime of `borrow1` ends
    {
        let borrow2 = &i;          // Lifetime of `borrow2` starts
    }                              // Lifetime of `borrow2` starts
});                                // Lifetime of `i` ends

/// ### Static Lifetime
/// The reserved lifetime `'static` tells that a definition will live from the
/// point of initialization to the end of the program execution.
runnable!(static_lifetimes, {
    /// #### Constants
    /// Constants have all 'static lifetime and are save in read-only memory.
    {
        const CONST_1: &'static u8 = &0u8;
        const CONST_2: &u8 = &0u8;

        static STATIC_1: &'static u8 = &0u8;
        static STATIC_2: &u8 = &0u8;
    }
    // <-- Here you cannot use `CONST_X` and `STATIC_X` anymore, but their
    //     values are still stored in memory
});

/// ### Explicit Lifetimes
/// In case the compiler cannot correctly infer the lifetime of a variable, you
/// must provide explicit type annotations.
/// 
/// Lifetime annotations are generic: a lifetime by itself does not mean much;
/// however you can express relations between lifetimes to constrain your
/// definitions.
runnable!(explicit_lifetimes, {
    /// #### Function/Method Lifetimes
    /// For functions/methods, the lifetime of the return must be equal or
    /// greater ('static) than an input parameter (by default the shortest).
    /// Otherwise we can be sure that the return references something inside the
    /// body of the function, which will result in a dangling pointer after
    /// returning.
    fn do_something<'lifetime>(){ }
    // ^ If the compiler cannot infer a lifetime, it defaults to 'static

    // fn supplier<'a>() -> &'a Box<u8> { &Box::new(0) }
    // ^ Error: 'a does not outlive the function

    fn print<'a>(x: &'a i32)
    { println!("{}", x); }

    fn add_one<'a>(x: &'a mut i32)
    { *x += 1; }

    fn print_two<'a, 'b>(x: &'a i32, y: &'b i32)
    { println!("{} {}", x, y); }

    fn identity1<'a>(x: &'a i32) -> &'a i32
    { x }

    // fn identity2<'a, 'b>(x: &'a i32) -> &'b i32
    // { x }
    // ^ Error: there is no guarantee that 'a is longer than 'b

    /// #### Lifetime Bounds
    /// Bounds can be used to specify lifetime contraints.
    fn identity3<'a: 'b, 'b>(x: &'a i32) -> &'b i32
    { x }
    // ^ Here 'a:'b means "'a must last longer than 'b"

    /// #### Implicit Lifetime Coercion
    /// Any lifetime is automatically coerced into a shorter lifetime when 
    /// needed
    fn require_sorted_by_lifetime
    <'a: 'b, 'b: 'c, 'c, A, B, C>(a: &'a A, b: &'b B, c: &'c C){}
    let x = &0;
    let y = &0;
    {
        let z = &0;
        require_sorted_by_lifetime(z, x, y);  // Error Expected
        require_sorted_by_lifetime(x, y, z);
        // ^ The function does not have the expected behavior because of 
        //   automatic coercion. You would expect the first call not to compile,
        //   but in reality the compiler automatically coerces the lifetimes of
        //   `x` and `y` to the one of `z` (therefore 'a = 'b = 'c). Coercion
        //   from a longer lifetime to a shorter one is always possible.
    }

    /// #### Structs Lifetimes
    /// For structures, the lifetime of an instance cannot outlive those of its
    /// fields. Otherwise, its fields may be dangling pointers.
    struct CStruct<'a> {
        field: &'a i32,
    };

    /// #### Methods Lifetimes
    /// Similar to functions.
    impl<'s> CStruct<'s> {
        fn static_method<'a>(x: &'a i32) -> &'a i32 { x }
        fn method(&'s self) -> &'s Self { self }
    }

    /// #### Generic Lifetimes
    /// Lifetime constraints can also be applied to generic types.
    struct Generic<'t, T: 't>{
        substruct: &'t T        
    }
    // ^ Here `substruct` has lifetime 't, just like before.
    //   However, there is the additional constraint of `T: 't`, which means
    //   that `T` is a structure where any inner reference must outlive 't.
    struct BoundedGeneric<'t, T: std::fmt::Display + 't>{
        substruct: &'t T
    }
    // ^ Standard type bounds can be mixed with lifetime bounds using `+`
});

/// #### Lifetime Elision
/// Most of the time, the compiler will be able to infer the shortest suitable
/// lifetimes through a process called `Lifetime Elision`, so you can omit them.
/// 
/// Elision uses inference rules based on empirically common patterns, namely:
/// 1. Any input parameter/field is assigned a different lifetime
/// 2. If there is only one input lifetime, it is assigned also to the outputs
/// 3. If an input is &self or &mut self, its lifetime is assigned to the output
runnable!(lifetime_elision, {
    fn unary1<'a>(s: &'a str) -> &'a str { s }
    fn unary2<'a>(s: &'a str) -> &str { s }
    fn unary3(s: &str) -> &str { s }

    fn binary1<'a, 'b>(x: &'a str, y: &'b str) -> &'a str { x }
    // fn binary2<'a, 'b>(x: &'a str, y: &'b str) -> &str { x }
    // ^ Error: cannot infer lifetime of output
    // fn binary3(x: &str, y: &'str) -> &str { x }
    // ^ Error: cannot infer lifetime of output

    struct A<'s> { string: &'s str };
    impl<'s> A<'s> {
        fn method1<'a>(&'s self, a: &'a str) -> &'s str { self.string }
        fn method2<'a>(&'s self, a: &'a str) -> &str { self.string }
        fn method3(&self, a: &str) -> &str { self.string }

        fn method4<'a>(&'s self, a: &'a str) -> &'a str { a }
        // fn method5<'a>(&'s self, a: &'a str) -> &str { a }
        // ^ Error: output expected to have lifetime 's
        // fn method6(&self, a: &str) -> &str { a }
        // ^ Error: output expected to have lifetime 's
    }
});