/// # Primitive Types
/// ## Signed Integers
const I8: i8 = 0;
const I8_1: i8 = 0i8;              // Suffix annotation
const I16: i16 = 0;
const I32: i32 = 0;                // Default
const I64: i64 = 0;
const I128: i128 = 0;
const ISIZE: isize = 0;            // Architecture-Based (either i32 or i64)

/// ## Unsigned Integers
const U8: u8 = 0;
const U8_2: u8 = 0b11111111u8;     // Binary notation
const U8_3: u8 = 0o377u8;          // Octal notation
const U8_4: u8 = 0xffu8;           // Hexadecimal notation
const U16: u16 = 0;
const U32: u32 = 0;
const U64: u64 = 0;
const U128: u128 = 0;
const USIZE: usize = 0;            // Architecture-Based (either u32 or u64)

/// ## Floating Points
const F32: f32 = 0.003;
const F64: f32 = 0.003;            // Default
const F64_1: f64 = 7.6e-3;         // Exponential Notation
const F64_2: f64 = 1_000.000_001;  // Underscore Notation

/// ## Chars
/// (unicode - 4 bytes)
const CHAR: char = 'a';
const CHAR_1: char = 'α';
const CHAR_2: char = '∞';

/// ## Booleans
const TRUE: bool = true;
const FALSE: bool = false;

/// ## Unit
const UNIT: () = ();

/// ## Nothing
/// The bottom type
fn nothing() -> ! { panic!("An exception has been thrown!") }
fn nothing_1() -> ! { loop {} }
fn nothing_2() -> ! { std::process::exit(0) }

/// ## String Slices
const STRING_SLICE: &str = "abc";

/// ## Arrays
/// (fixed length sequences - compile-time checks)
const ARRAY: [i32; 5] = [0; 5];            // [0, 0, 0, 0, 0]
const ARRAY_1: [i32; 5] = [1, 2, 3, 4, 5];
const ARRAY0: i32 = ARRAY[0];
const ARRAY1: i32 = ARRAY[1];
const ARRAY2: i32 = ARRAY[2];
const ARRAY3: i32 = ARRAY[3];
const ARRAY4: i32 = ARRAY[4];
const ARRAY_LEN: usize = ARRAY.len();

/// ## Array Slices
/// (dynamic length sequences - run-time checks)
fn slice() { let slice: &[i32] = &ARRAY[2 .. 4]; }

/// ## Tuples
/// (only printable with <=12 elements)
const TUPLE: (i32, char, bool) = (1, 'a', true);
const TUPLE0: i32 = TUPLE.0;
const TUPLE1: char = TUPLE.1;
const TUPLE2: bool = TUPLE.2;