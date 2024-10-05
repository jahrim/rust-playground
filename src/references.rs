/// # References
/// Memory in Rust is split into `stack`, for local statically-allocated
/// variables, and `heap`, for global dinamically-allocated variables.

/// ## Stack References
/// All values in Rust are stack-allocated by default.
/// 
/// Note: Rust provides a low-level primitive for references, called Pointers.
///       This can be used for manual memory-management beyond the compiler's
///       capabilities.
runnable!(stack_references, {
    /// Address Type and Operator
    let immutable_reference_to_immutable: &u8 = &0u8;
    let mut mutable_reference_to_immutable: &u8 = &0u8; 

    let immutable_reference_to_mutable: &mut u8 = &mut 0u8;
    let mut mutable_reference_to_mutable: &mut u8 = &mut 0u8; 

    /// Dereference Operator
    let value: u8 = *immutable_reference_to_immutable;
    let value: u8 = *mutable_reference_to_immutable;
    let value: u8 = *immutable_reference_to_mutable;
    let value: u8 = *mutable_reference_to_mutable;
    
    let reference: &mut u8 = &mut 1;
    // immutable_reference_to_immutable = reference;
    // ^ Error: cannot change immutable variable
    mutable_reference_to_immutable = reference;
    // immutable_reference_to_mutable = reference;
    // ^ Error: cannot change immutable variable
    mutable_reference_to_mutable = reference;

    // *immutable_reference_to_immutable += 10;
    // ^ Error: cannot change data immutably referenced
    // *mutable_reference_to_immutable += 10;
    // ^ Error: cannot change data immutably referenced
    *immutable_reference_to_mutable += 10;
    *mutable_reference_to_mutable += 10;
});

/// ## Heap References
/// You can explicitly allocate some value in the heap using the `Box` wrapper.
runnable!(heap_references, {
    let immutable_reference_to_immutable: Box<u8> = Box::new(0);
    let mut mutable_reference_to_immutable: Box<u8> = Box::new(0);

    let value: u8 = *immutable_reference_to_immutable;
    let value: u8 = *mutable_reference_to_immutable;
    
    let reference: Box<u8> = Box::new(1);
    // immutable_reference_to_immutable = reference;
    // ^ Error: cannot change immutable variable
    mutable_reference_to_immutable = reference;
});

/// ## Dereference Coercion (~ Scala Implicit Conversions, only for references)
/// Rust can automatically convert references of some type into other references
/// of some other type, provided an implementation of the
/// `std::ops::Deref.deref(&self) -> &Self::Target` trait.
/// 
/// In particular, `Box<A>` implement the `Deref` trait, such that you can
/// actually use a `&Box<A>` any time you require an `&A`.
use std::ops::{Deref, DerefMut};

runnable!(dereference_coercion, {
    let x: Box<u8> = Box::new(0);
    let y: &u8 = x.deref();         // Explicit
    let y: &u8 = &x;                // Implicit
    println!("x={} y={}", *x, *y);
});

runnable!(ownership_and_dereference_coercion, {
    let mut x: Box<u8> = Box::new(0);
    let y: &mut u8 = x.deref_mut(); // Explicit
    let y: &mut u8 = &mut x;        // Implicit
    // <-- Ownership of `x` is moved to `y`

    *y += 1;
    // *x += 1;
    // ^ Error: use of `x` while mutably borrowed
    println!("y={}", *y);
    // <-- Ownership of `y` is moved to `x`
    //     (`y` is not used later in the code)

    *x += 1;
    // *y += 1;
    // ^ Error: `y` would keep borrowing `x`, so updating `x` becomes illegal
    println!("x={}", *x);
});