/// # Module Definition
/// Modules contain `struct`, `impl`, `fn`, `const`, `static` or other modules.
/// 
/// Inside a module, visibility constraint can be used to tell where elements
/// can be accessed from.
mod module_name {
    fn private() {}
    pub fn public() {}

    mod public_submodule {
        /// ## Built-In Scopes
        pub(self) fn private() {
            public_inside_parent();                  // Use `self` definition
            self::public_inside_parent();            // Use `self` definition
        }
        pub(super) fn public_inside_parent() {
            super::public();                         // Use `super` definition
        }
        pub(crate) fn public_inside_this_crate() {
            crate::modules::module_name::private();  // Use `crate` definition
        }
        pub(in crate::modules::module_name) fn public_inside_module_name() {
            // crate::module_name must be a path to an ancestor
        }
    }

    pub struct PublicStruct {
        private_field: f64,
        pub public_field: f64,
    }

    impl PublicStruct {
        pub fn public_factory_method() -> PublicStruct {
            PublicStruct::private_factory_method(0.0, 0.0)
        }
        fn private_factory_method(a: f64, b: f64) -> PublicStruct {
            PublicStruct { private_field: a, public_field: b }
        }
    }
    
    /// ## Exporting
    /// A module can export public members from other modules. Exports can also
    /// be aliased.
    mod moduleA { 
        mod moduleB { pub fn functionB(){} }
        pub use moduleB::functionB;  
        // ^ Copy `moduleB::functionB` to `moduleA` as `moduleA::functionB`
    }
    pub use moduleA::functionB as fb;
    // ^ Copy `moduleA::functionB` to `module_name` as `module_name::fb`

}

/// ## File Module Definition
/// Locate a file named `modules/submodule.rs` and creates a module wrapping its
/// content.
/// 
/// If used in the crate root (e.g., src/main.rs), it searches files in the
/// crate directory (e.g., src).
mod submodule;

/// ## Directory Module Definition
/// Locate a directory named `modules/submodule2` and creates a module wrapping
/// the content of the file `modules/submodule2/mod.rs`.
mod submodule2;