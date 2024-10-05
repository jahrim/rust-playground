/// # Unsafety
/// Rust allows developer to have full control of their program, disabling
/// compiler checks. However, this requires specific annotations from both the
/// developer of the API and the API user.
mod developer {
    /// ## Unsafe Function Definition
    /// An unsafe function must be declared as `unsafe`.
    pub unsafe fn unsafe_function(){ /* do some magic */ }
}
runnable!(user, {
    /// ## Unsafe Scope
    /// In order to use unsafe functions, the user must create an explicit
    /// unsafe scope.
    unsafe {
        let x: () = developer::unsafe_function();
        /* do some other magic */
    }
});

/// ## Raw Pointers
/// Rust references are checked so they always point the a valid memory address.
/// However, Rust allows developers to use unsafe `raw pointers` (like C).
runnable!(raw_pointers, {
    let reference: &u32 = &10;      // Reference: safe to declare; safe to use
    let pointer: *const u32 = &10;  // Pointer: safe to declare; unsafe to use

    assert!(*reference == 10);
    // assert!(*pointer == 10);
    // ^ Error: derefencing a pointer is an unsafe operation
    unsafe { assert!(*pointer == 10); }
});

/// ## Assembly
/// Rust allows developers to write inline assembly code for implementing the
/// most efficiency-critical parts of their code.
/// 
/// See https://doc.rust-lang.org/rust-by-example/unsafe/asm.html.
use std::arch::asm;

runnable!(assembly, {
    let x: u64;
    unsafe { asm!("mov {}, 5", out(reg) x); }
    assert_eq!(x, 5);
});

/// ## Foreign Function Interface (FFI) - C
/// Rust can leverage functions implemented for C using the `extern` keyword.
/// These functions are unsafe to call, so it is a common practice to wrap them
/// into safe Rust code, creating Foreign Function Interfaces.
///
/// You can leverage conditional compilation for os interoperability.
// Declare the following C-like structure
#[repr(C)]
#[derive(Clone, Copy)]
struct Complex { re: f32, im: f32, }

#[cfg(target_family = "windows")]   // if you are on windows
#[link(name = "msvcrt")]            // from the `msvcrt` C library
extern {
    // v-------------------------------take the following foreign functions
    fn csqrtf(z: Complex) -> Complex;
    fn ccosf(z: Complex) -> Complex;
}

#[cfg(target_family = "unix")]
#[link(name = "m")]
extern {
    fn csqrtf(z: Complex) -> Complex;
    fn ccosf(z: Complex) -> Complex;
}

// Foreign Function Interface
fn cos(z: Complex) -> Complex { unsafe { ccosf(z) } }