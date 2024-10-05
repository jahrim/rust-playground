/// # Structs

/// ## Unit Structs
struct Unit;
const UNIT: Unit = Unit;

/// ## Tuple Structs
/// Tuple Structs are useful for abstracting from primitive tuples.
struct Years(u64);
struct Pair(i32, f32);
const PAIR: Pair = Pair(0, 1.3);
const PAIR0: i32 = PAIR.0;
const PAIR1: f32 = PAIR.1;

/// ## C Structs
#[derive(Debug)]    // Automatically derive trait `std::fmt::Debug`
struct Person {
    id: u64,
    age: u8,
}
const PETER: Person = Person {
    id: 0,
    age: 27,
};
const PETER_ID: u64 = PETER.id;
const PETER_AGE: u8 = PETER.age;

/// ## Structural Update
/// Rust allows a structure to inherit undefined fields from another structure.
const JOHN: Person = Person {
    id: 1,
    ..PETER         // everything else (i.e. `age`) is imported from PETER
};

/// ## Destructuring
/// Structures can be destructured in their individual components.
runnable!(destructuring, {
    /// ### Destructuring Tuples
    let (x, y) = (0, 'a');
    println!(".0={} .1={}", x, y);

    /// ### Destructuring Tuples Structs
    let Pair(x, y) = PAIR;
    println!(".0={} .1={}", x, y);

    /// ### Destructuring Structs
    let Person { id: x, age: y } = PETER;
    println!("id={} age={}", x, y);

    /// ### Let-Else - Handling Destructuring Failures
    let person: Person = Person { id: 2, age: 20 };
    let Person { id: 3, age: person_age } = person else {
        panic!("Pattern match failed on variable '{person:?}': id mismatch");
    };
});