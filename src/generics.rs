/// # Generics
/// Generics are a mechanism to abstract functionalities to broader cases.
fn generics() {}

/// ## Generic Structs
/// Generic structs are structs which depends on a type that is not known at
/// the time of the definition (however, it is always known when instantiated).
struct Concrete;
struct ConcreteWrapper(Concrete);
struct GenericWrapper<Generic>(Generic);
// ^ Here `Generic` is a generic type parameter. Since `Generic` is generic,
//   then also its wrapper `GenericWrapper` is generic.

const CONCRETE_1: Concrete = Concrete;
const CONCRETE_2: ConcreteWrapper = ConcreteWrapper(Concrete);
const CONCRETE_3: GenericWrapper<i32> = GenericWrapper(10);

/// ## Generic Functions
/// Generic functions takes type parameters in input in addition to the standard
/// parameters.
fn concrete1() {}
fn concrete2(x: u8) {}
fn concrete3(x: GenericWrapper<u8>) {}
fn generic1<A>(x: A) {}
fn generic2<A, B>(x: A) -> B { panic!("not implemented") }
fn generic3<A>(x: GenericWrapper<A>) {}

/// ## Generic Function Application
/// Input type arguments can be specified using the `turbofish` syntax.
runnable!(generic_call, {
    generic1::<u8>(0);            // Explicit Type Argument
    generic1(0u8);                // Inferred Type Argument
    generic2::<u8, f64>(0);       // Here you cannot infer the type argument
});

/// ## Generic Methods
/// Generic methods can be defined similarly to functions.
impl<A> GenericWrapper<A> {
    fn unwrap(&self) -> &A { &self.0 }
}
// You can also provide methods for concrete types of generics
impl GenericWrapper<u8> {
    fn unwrap_to_u64(&self, out: &mut u64) { 
        *out = *self.unwrap() as u64;
    }
}

/// ## Generic Trait (~ Haskell Generic Type-Classes)
/// Generic traits can be defined similarly to functions.
trait Conversion<To> {
    fn convert(&self, out: &mut To) {}
}
impl<From, To> Conversion<To> for From {
    fn convert(&self, out: &mut To) { panic!("not implemented") }
}

/// ## Type Bounds
/// Type parameters can be bound to certain traits, so that you apply a function
/// to types whose implementation for those traits is in scope.
/// 
/// A type bound is specified with `:`. Multiple type bounds can be concatenated
/// with `+`.
fn drop1<A: Sized>(_: A) {}
fn drop2<A>(_: A) where A: Sized {}
fn drop_both<A: Sized, B: Sized>(_: A, _: B) {}

// Multiple bounds for the same type parameter
fn print_and_drop<A: Sized + std::fmt::Display>(x: A) {
    println!("{}", x);
}
// `where` clauses are more expressive for declaring type bounds
fn print_option_and_drop<A: Sized>(x: A)
where Option<A>: std::fmt::Display {
    println!("{}", Some(x));
}

/// ## Associated Items (~ Scala Type Members)
/// Traits can define inner types. These are useful to reduce repetitions.
/// - If the generic type is extrinsic from the object, better use type
///   parameters
///   Example: Conversion<To> (`To` is not part of the object)
///   Note: if you were to put `To` as a type member, you could only define one
///         `Conversion` per type in the same scope.
/// - If the generic type is intrinsic in the object, better use type members
///   Example: Wrapper<A> (`A` is part of the object)

// Without Type Members
trait Wrapper1<A> {
    fn get(self) -> A;
}
impl <A> Wrapper1<A> for Option<A> {
    fn get(self) -> A {
        match self {
            Some(x) => x,
            None => panic!("no such element")
        }
    }
}
fn consume_wrapper1<A, B: Wrapper1<A>>(x: B) -> A { x.get() }

// With Type Members
trait Wrapper2 {
    type Inner;                     // Type member
    fn get(self) -> Self::Inner;
}
impl <A> Wrapper2 for Option<A> {
    type Inner = A;
    fn get(self) -> Self::Inner {
        match self {
            Some(x) => x,
            None => panic!("no such element")
        }
    }
}
fn consume_wrapper2<A: Wrapper2>(x: A) -> A::Inner { x.get() }  // more readable

/// ## Phantom Types
/// Phantom Types can be declared using `std::marker::PhantomData`.
/// 
/// These types are present only at compile-time for static analysis. In fact,
/// they are removed from the code at runtime, so they have no behavior and
/// waste no extra memory.
/// 
/// Example: it is useful to keep track of measurement units without overhead
use std::marker::PhantomData;
#[derive(PartialEq)]
struct Phantom<A, Marker> { 
    value: A, 
    marker: PhantomData<Marker>,
}
impl<A> Phantom<A, ()> {
    fn new<Marker>(x: A) -> Phantom<A, Marker> {
        Phantom { value: x, marker: PhantomData }
    } 
}
runnable!(phantom_types, {
    mod Markers { 
        #[derive(PartialEq)] pub struct Red; 
        #[derive(PartialEq)] pub struct Blue; 
    }
    let p1 = Phantom::new::<Markers::Red>(0u8);
    let p2 = Phantom::new::<Markers::Blue>(0u8);
    // println!("{}", p1 == p2) // Error: type mismatch
});