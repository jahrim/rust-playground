/// # Macros
/// Macros are a way to do meta-programming in Rust. They can be define using
/// the keyword `macro_rules!`. Calling a macro is just like calling a function,
/// but the name is followed by a `!` (e.g., `println!` is a macro).
fn macros(){}

/// ## Inlining
/// The most simple use of macros is for inlining. In fact, every time a macro
/// is called, the AST of its body its copied in the location of the code. This
/// reduce the additional overhead of calling a new function and managing its
/// stack.
macro_rules! hello_world {
    () => {
        println!("Hello world!")
    };
}

runnable!(inlining, {
    hello_world!();
    // ^ This will be expanded to `println!("Hello world!")` during compilation

    // You can also used different brackets for the input arguments
    hello_world![];
    hello_world!{}  // note: `;` is optional with `{}` brackets
});

/// ## Designators
/// A macro works by modifying the AST of the program, and to do so it takes
/// parts of the AST as inputs.
/// 
/// Inputs in macros are prefixed with `$`. `$` is also the evaluation operator,
/// which turns an expression into its result (careful because every time `$` is
/// called, the expression is re-evaluated).
/// 
/// Nodes in the AST corresponds to the possible semantic elements in the Rust
/// programming language, namely:
/// - `expr`: an expression
/// - `block`: an expression that creates a new scope
/// - `ident`: a variable or function name
/// - `item`
/// - `literal`: a literal constant
/// - `pat`: a pattern (pattern-matching)
/// - `stmt`: a statement
/// - `tt`: a token tree
/// - `ty`: a type
/// - `vis`: a visibility qualifier
macro_rules! eval {
    ($expression: expr) => (
        println!("{:?} -> {:?}", 
                 stringify!($expression), // the string code of `expression`
                 $expression);            // the evaluation of `expression`
    );
}
macro_rules! new_fn {
    ($fn_name: ident) => (
        fn $fn_name() { 
            println!("Called {:?}", stringify!($fn_name));
        }
    );
}

runnable!(designators, {
    eval!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
    new_fn!(ciao);
    ciao();
});

/// ## Macro Overload
/// The inputs to a macro don't need to be separated by a comma. In fact, they
/// can be separated by any template (some specific rules may apply).
/// 
/// These templates can be useful to build DSLs, defining new keywords in the
/// scope of the macro.
/// 
/// Inputs are extracted in a way similar to pattern matching. This allows to
/// define the same macros multiple times for different input patterns.
macro_rules! check {
    // ($lhs: expr and $rhs: expr) => {};  // Error: not allowed
    (not $lhs: expr) => ( !$lhs );
    ($lhs: expr; and $rhs: expr) => ( $lhs && $rhs );
    ($lhs: expr; or $rhs: expr) => ( $lhs || $rhs );
}
runnable!(overloading, {
    println!("and: {}", check!(true; and false));
    println!("or: {}", check!(true; or false));
    println!("not: {}", check!(not true));

    // Careful: macros are not necessarily typed, which may lead to unintended
    // behaviors. Here !100u8 is computed as 255u8 - 100u8 = 155u8.
    println!("not: {}", check!(not 100u8));
});

/// ## Typed Macros
/// You can enforce the types of input expressions in macros, by assigning
/// their results to typed variables.
/// 
/// Since macros are expanded before type-checking the program, the compiler
/// can still tell if the expanded program is correctly typed.
macro_rules! bool_check {
    (not $lhs: expr) => ({ let typed_lhs: bool = $lhs; typed_lhs });
    ($lhs: expr; and $rhs: expr) => ({
        let (tlhs, trhs): (bool, bool) = ($lhs, $rhs);
        tlhs && trhs
    });
    ($lhs: expr; or $rhs: expr) => ({
        let (tlhs, trhs): (bool, bool) = ($lhs, $rhs);
        tlhs || trhs
    });
}
runnable!(typed_macros, {
    println!("and: {}", bool_check!(true; and false));
    println!("or: {}", bool_check!(true; or false));
    println!("not: {}", bool_check!(not true));
    // println!("not: {}", bool_check!(not 100u8));
    // ^ Error: expected `bool`, found `u8`
});

/// ## Variadic Macros
/// Macros can take variable arguments as inputs in three ways:
/// - `?`: indicates that an input can be repeated 0..1 times
/// - `*`: indicates that an input can be repeated 0..N times
/// - `+`: indicates that an input can be repeated 1..N times
/// 
/// For each symbol, the available templates are `$(PATTERN)?` (no separator),
/// `$(PATTERN),?` (comma separator) and `$(PATTERN);?` (semi-colon separator).
/// 
/// Note: macros can also be defined recursively.
macro_rules! calculator {
    (eval $exp:expr) => ({
        let result: usize = $exp;
        println!("{} = {}", stringify!($exp), result);
    });
    (eval $exp:expr; $(eval $exp_i: expr);*) => ({
        calculator!(eval $exp);         // call on head
        calculator!($(eval $exp_i);*);  // call on tail
    }) 
}
runnable!(variadic_macros, {
    calculator!{
        eval 1;
        eval 1 + 3;
        eval 5 + 8;
        eval (20 - 13) + 32
    };
});

/// ## Macros in Libraries
/// Macros are treated specially by the compiler. In particular, it is not
/// possible to define them as public. However, there are a few ways to export
/// them.
mod module {
    /// ### Macro Export
    /// A module can export its members using Rust export syntax, which also
    /// works for macros.
    mod submodule1 {
        macro_rules! my_macro1 { () => { println!("my_macro1"); }; }
        pub(crate) use my_macro1;
        // pub macro_rules! my_macro2 { () => { "my_macro2" }; }
        // ^ Error: visibilities are not allowed on macros
    }

    /// ### Global Macro Export
    /// A module marked with `macro_use` will move all of its macro definitions
    /// to the global scope.
    /// 
    /// Note: these macro definitions will only be available after the module
    /// is defined in the scope (i.e. order of definitions matter).
    #[macro_use]
    mod submodule2 {
        macro_rules! my_macro3 { () => { println!("my_macro3"); }; }
    }
    
    /// ### Macro Import
    /// You can import macros from modules. No need to import global macros.
    runnable!(macro_import, {
        use submodule1::my_macro1;
        my_macro1!();
        my_macro3!();
    });
}