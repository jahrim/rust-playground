/// # Assignments
/// All assignments are statements and must end in `;`.

/// ## Global Immutable Values
const CONSTANT: i32 = 10;

/// ## Global Mutable Variables
/// Immutable memory address
static STATIC: &str = "string";

/// ## Local Variables
/// Local variables can only be declared inside a scope, by using the keyword
/// `let`.
runnable!(local_variables, {
    /// Immutable Local Variable
    let immutable_var: i32 = 1;
    // immutable_var += 1;
    // ^ Error: cannot modify immutable variable

    /// Mutable Local Variable
    let mut mutable_var: i32 = 1;
    mutable_var += 1; 

    /// Scopes
    let var: i32 = 1;
    {
        let scoped_var: String = String::from("string");
        println!("var: {} scoped_var: {}", var, scoped_var);
    }
    // println!("{} {}", var, scoped_var);
    // ^ Error: undefined variable `scoper_var`

    /// Shadowing
    let shadowed_var: i32 = 1;
    {
        let mut shadowed_var: String = String::from("string");
        println!("shadowed_var (in): {}", shadowed_var);
    }
    let shadowed_var: char = 'a';
    println!("shadowed_var (out): {}", shadowed_var);

    /// Declaration & Definition (rarely used)
    let declared_var: i32;
    // println!("{}", declared_var);
    // ^ Error: uninitialized variable `declared_var`

    declared_var = 1;
    println!("declared_var: {}", declared_var);
});