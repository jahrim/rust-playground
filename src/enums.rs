/// ## Enum Types (~ Scala Enums, no type hierarchy though)
/// An Enum Type is a type which can be one of different kinds of structs.
/// 
/// Note: the possible kinds of an Enum Type are called `variants`. These
///       behave differently from actual types (e.g., they cannot be used as a
///       type by themselves).
enum WebEvent {
    PageLoaded,                 // Unit Structs
    PageUnloaded,
    KeyPressed(char),           // Tuple Structs
    Copy(String),               
    Paste(String),
    Clicked { x: i64, y: i64 }  // C Structs
}
const PAGE_LOADED: WebEvent = WebEvent::PageLoaded;
// const PAGE_UNLOADED: WebEvent::PageUnloaded = WebEvent::PageUnloaded;
// ^ Error: expected type, found variant `WebEvent::PageUnloaded`

/// ## Pattern Matching
/// An Enum Type can be matched againts its possible variants.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoaded => println!("loaded"),
        WebEvent::PageUnloaded => println!("unloaded"),
        WebEvent::KeyPressed(key) => println!("key {} pressed", key),
        WebEvent::Copy(string) => println!("{} copied", string),
        WebEvent::Paste(string) => println!("{} pasted", string),
        WebEvent::Clicked { x, y } => {
            println!("({},{}) clicked", x, y)
        },
    }
}

/// ## Implicitly Discriminated Enums
/// Unit structs in enum types are assigned a unique value. By default, values
/// starts from `0i32`.
enum Number {
    Zero,               // same as Zero = 0
    One,                // same as One = 1
    Two,                // same as Two = 2
}

/// ## Explicitly Discriminated Enums
/// You can also specify your own values.
enum Color {
    Red = 0xff0000,     // rgb: [256, 0, 0]
    Green = 0x00ff00,   // rgb: [0, 256, 0]
    Blue = 0x0000ff,    // rgb: [0, 0, 256]
}

/// ## Enum Value
/// Finally, you can extract the value of a variant through casting.
const ZERO_VALUE: i32 = Number::Zero as i32;
const RED_VALUE: i32 = Color::Red as i32;