/// # Traits (~ Haskell Type Classes)
/// A trait is a contract containing methods and type members.
/// 
/// A trait can be implemented for any type in Rust, making its contract
/// available for the instances of that type.
runnable!(traits, {
    /// ## Trait Definition
    trait Animal {
        /// Static Method
        fn new() -> Self;

        /// Method
        fn noise(&self) -> &'static str;

        /// Default Method
        fn make_noise(&self) { println!("{}", self.noise())}
    }

    /// ## Trait Implementation
    /// The implementation of a trait for a struct is separated from the
    /// definition and implementation of the struct.
    struct Dog;
    impl Animal for Dog {
        // <-- Here `Self` is a `Dog` (even as a constructor)
        fn new() -> Self { Dog }
        fn noise(&self) -> &'static str { "Woff" }
    }
    struct Cat;
    impl Animal for Cat {
        // <-- Here `Self` is a `Cat` (even as a constructor)
        fn new() -> Self { Self }  // Calling `Self` as a constructor
        fn noise(&self) -> &'static str { "Miao" }
    }

    // Calling trait static methods require the type to be inferrable
    let (dog, cat): (Dog, Cat) = (Animal::new(), Animal::new());

    /// ## Trait Bounds
    /// Traits can be used as a type bound in generics to work with types that
    /// satisfy a certain contract.
    fn make_noise<A: Animal>(animal: A) {
        animal.make_noise();
    }

    /// ## Trait Derivation
    /// The Rust compiler is capable of providing basic implementation for some
    /// built-in traits, namely for:
    /// - Equality: `Eq`, `PartialEq`, `Ord`, `PartialOrd`, for comparing values
    /// - Cloning: `Clone`, for creating `T` from `&T`
    /// - Copying: `Copy`, for changing *move semantics* to *copy semantics*
    /// - Hashing: `Hash`, for computing the hash of a reference `&T`
    /// - Construction: `Default`, for creating empty instances of a type `T`
    /// - Printing: `Debug`, for formatting values into human-readable strings
    /// 
    /// All these traits can of course be manually implemented for specific
    /// use cases.
    #[derive(Debug, PartialEq, PartialOrd)]
    struct Unit;
    #[derive(Debug, PartialEq, PartialOrd)]
    struct Measure { amount: f64, unit: Unit, };

    /// ## Trait Objects
    /// Rust requires to know the size in memory of each variable. As a
    /// consequence, traits cannot be used as a type directly, because every
    /// implementation has different memory requirements.
    /// 
    /// To use traits as types, you can use *references* to traits, as all
    /// references have the same size (i.e. the size of a pointer). However,
    /// this only works for *trait objects* (i.e. object-safe traits), which
    /// support *dynamic dispatch*. This constraint must also be marked in the
    /// type using the keyword `dyn`.
    /// 
    /// See: https://doc.rust-lang.org/reference/items/traits.html#object-safety
    trait HasNoise { fn noise(&self) -> &'static str; }  // Trait Object

    impl HasNoise for Cat { fn noise(&self) -> &'static str { "Miao" } }
    impl HasNoise for Dog { fn noise(&self) -> &'static str { "Dog" } }
    let has_noise: &dyn HasNoise = &Cat;

    // let animal: &dyn Animal = &Cat;
    // ^ Error: `Animal` is not a trait object (i.e. not object-safe):
    //          Animal::new() -> Self cannot be dynamically dispatched

    /// Trait objects can be useful to deal with containers of values with
    /// different types, all adhering to a common contract
    let mut has_noise: Box<dyn HasNoise> = Box::new(Cat);
    println!("{}", has_noise.noise());
    has_noise = Box::new(Dog);
    println!("{}", has_noise.noise());

    /// ## Anonymous Trait Implementations
    /// A different way to generalize from concrete instances is using
    /// anonymous trait implementations, which do not require traits to be
    /// object-safe, however they can only be used in function declarations.
    fn animal() -> impl Animal { Cat }

    /// While this mechanism has less constraints on the definition of the
    /// trait, it has more limitations on how it can be used.
    // fn new_noise2(is_dog: bool) -> impl HasNoise {
    //    if is_dog { Dog } else { Cat }  
    // }
    // ^ Error: `Dog` and `Cat` have incompatible types for `if-then-else`

    // The same function can be implemented for `trait-objects`
    fn new_noise3(is_dog: bool) -> Box<dyn HasNoise> {
        if is_dog { Box::new(Dog) } else { Box::new(Cat) }
    }

    /// ## Operator Overloading
    /// Rust provides some built-in traits to overload primitive operators
    /// (i.e., `+`, `-`, ...). These traits are located in `std::ops`.
    use std::ops::Add;
    impl Add<Measure> for Measure {
        type Output = Result<Measure, ()>;
        fn add(self, rhs: Measure) -> Self::Output { 
            if self.unit == rhs.unit {
                let sum: f64 = self.amount + rhs.amount;
                Ok(Self { amount: sum, unit: self.unit })
            } else {
                Err(())
            }
        }
    }
    let m1: Measure = Measure { amount: 1.0, unit: Unit };
    let m2: Measure = Measure { amount: 2.0, unit: Unit };
    println!("x={:?} y={:?}", m1, m2);
    println!("x+y={:?}", m1 + m2);

    /// ## Supertraits
    /// In Rust there is no concept of inheritance or subtyping, but you can
    /// define trait as extensions of other traits, that is `supertraits`.
    /// 
    /// Simply put, a sub-trait must also adhere to the contracts of its
    /// super-traits.
    trait Person {
        fn name(&self) -> String;
    }
    trait Programmer { 
        fn name(&self) -> u8;
        fn main_language(&self) -> String;
    }

    // A `Student` contains the properties of a `Person`. This implies that an
    // implementation of `Person` must be provided before implementing `Student`
    trait Student: Person { fn university(&self) -> String; } 

    // A `ComputerScienceStudent` contains the properties of both a `Student`
    // and a `Programmer`. This implies that an implementation of both `Student`
    // and `Programmer` must be provided before implementing
    // `ComputerScienceStudent`
    trait ComputerScienceStudent: Student + Programmer {}

    impl Person for Cat {
        fn name(&self) -> String { String::from("Miao?") }
    }
    impl Student for Cat {
        fn university(&self) -> String { String::from("MiaoMiao") }
    }
    impl Programmer for Cat { 
        fn name(&self) -> u8 { 0b10101010 }
        fn main_language(&self) -> String { String::from("Miaooo") }
    }
    impl ComputerScienceStudent for Cat {}

    /// ## Trait Disambiguation
    /// Since all implementations are distinct, there cannot be conflicts in
    /// overlapping traits. However, some methods calls may be ambiguous for
    /// the compiler and require the developer to disambiguate the type.
    let cat = Cat;
    // println!("{}", cat.name());
    // ^ Error: ambiguous call: do you mean Person::name or Programmer::name?
    println!("Cat Person - Name: {}", <Cat as Person>::name(&cat));
    println!("Cat Programmer - Name: {}", <Cat as Programmer>::name(&cat));
    println!("Cat University: {}", cat.university());
    println!("Cat Main Language: {}", cat.main_language());
});