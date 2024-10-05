enum Stage { Beginner, Advanced }
enum Role { Student, Teacher }

/// # Imports
/// Imports allow to use relative names instead of canonical names for
/// referencing definitions. It works with modules and enums.
runnable!(imports, {
    /// Relative Import, Multi-Import, and Aliasing
    use Stage::{Beginner, Advanced as Adv};
    // <-- Here, `Beginner` is the same as `Stage::Beginner` and `Adv` is the
    //     same as `Stage::Advanced`
    /// Absolute Import, and Import-All
    use crate::imports::Role::*;
    // <-- Here, any `Role::X` can be referenced as `X`

    let stage: Stage = Beginner;
    match stage {
        Beginner => println!("Beginners are starting their learning journey!"),
        Adv => println!("Advanced learners are mastering their subjects..."),
    }

    let role: Role = Student;
    match role {
        Student => println!("Students are acquiring knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }
});