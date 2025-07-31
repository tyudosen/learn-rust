fn main() {
    // Rust has:
    // (I) Scalar Data Type.
    // (II) Compound Data Type.

    // Rust compiler can infer most types based off use
    // but in cases where multiple types are possible
    // type annotation is required.

    // (I) Scalar Types
    // Represents a single value

    // (a) Integer Types
    let a: i32 = 1; // By default rust has an i32 integer type
    let b: i8 = 2;
    let c: i16 = 3;
    let d: i64 = 4;
    let e: i128 = 5;
    let f: isize = 6; // size is system architecture dependent.

    // `i` here represents a signed integer type and the number
    // is the number of bits the integer takes in memory.
    // The range of numbers a signed integer can take are:
    // -(2^(n-1)) ... 2^(n-1) -1 (end range is included).
    // Signed numbers are stored using **two's complement**
    // representation.

    let g: u8 = 1; // By default rust has an i32 integer type
    let h: u16 = 2;
    let i: u32 = 3;
    let j: u64 = 4;
    let k: u128 = 5;
    let l: usize = 6; // size is system architecture dependent

    // `u` here represents a unsigned integer type and the number
    // is the number of bits the integer takes in memory.
    // The range of numbers a signed integer can take are:
    // 0 ... 2^(n) -1 (end range is included)

    // Integer types can be written in the form of:
    // (i) Decimal => 98_222 or 98222
    // (ii) Hex => 0xff
    // (iii) Octal => 0o77
    // (iv) Binary => 0b1111_0000
    // (v) Byte (u8 only) => b'A'

    // In the case of an integer overflow. Trying to put a value
    // outside the range a specific type can hold. One of two things
    // could happen.
    // (i) In debug mode: Rust will include  hecks for integer
    // overflows and will panic, crashing the program if there is
    // one.
    // (ii) In release mode (with --release flag): Rust does not
    // include checks and will not cause the program to panic.
    // Instead Rust performs two's complement wrapping. This will
    // wrap around to the start of the range.

    //  To handle the possibility of overflow:
    //  (i) Wrap in all modes with the wrapping_* methods,
    //  such as wrapping_add .
    //  (ii) Return the None value if there is overflow with
    //  the checked_* methods.
    //  (iii) Return the value and a Boolean indicating whether
    //  there was overflow with the overflowing_* methods.
    //  (iv) Saturate at the value’s minimum or maximum values
    //  with the saturating_* methods.

    // (b) Floating-Point Types
    let m = 2.0; // Default floating point type is `f64`.

    // `f64` on modern CPUs is roughly the same speed as f32
    // but is capable of more precision.

    let n: f32 = 3.0;

    // They are represented according to the IEEE-754 standard.

    // (c) Numeric Operations
    // Supports basic numeric operations
    let a = 1;
    let b = 2;

    // addition
    let sum = a + b;

    // multiplication
    let difference = a - b;

    // division
    let quotient = 56.7 / 32.2;

    // Integer division truncates towards zero to the nearest
    // integer.
    let truncated = -5 / 3; // Result => -1

    // remainder
    let remainder = 43 % 5;

    // (c) The Boolean Type
    let bool_val = true;
    let bool_val: bool = false;

    // (d)  The Character Type
    // Uses single quotes as opposed to the double quotes
    // used in regular strings.
    // Char is 4 bytes in size and represents a unicode
    // scalar value. This allows it to represent more than ASCII.
    // Accented letters; Chinese, Japanese, and Korean characters;
    // emoji; and zero-width spaces are all valid char
    // values in Rust.
    // Unicode scalar values range from U+0000 to U+D7FF
    // and U+E000 to U+10FFFF inclusive.

    let char_val = 'z';
    let char_val: char = 'Z';
    let emoji_char = '😻';

    // (II) Compound Types
    // Can group multiple values into one type.

    // (a) The Tuple Type
    // Groups together different values with a variety of types.
    // Fixed length. Can not grow or shrink once decalred.

    let tup = (500, true, 6.4);
    let tup: (bool, i32, u32) = (true, -20, 30);

    // Pattern matching can be used to extract values
    let (foor, bar, bin) = tup;

    // Values can also be accessed using a period (.) notation
    let foor = tup.0;
    let bar = tup.1;
    let bin = tup.2;

    // You can change the values of a mutable tuple.
    // You can only change to the same type that was used
    // when that value was declared.
    let mut mutable_tup = (1, 2);
    mutable_tup.0 = 10;
    mutable_tup.1 = 20;
    mutable_tup.0 = true; // Mismatched types error

    // (b) The Array Type
    // Holds a collection of values.
    // Every value must be of the same type
    // Has a fixed length defined at declaration.

    let rust_array = [1, 2, 3, 4];
    let first = rust_array[0];
    let second = rust_array[1];

    // explicit type annotation
    let rust_array: [i32; 3] = [1, 2, 3];

    // intitialize to contain the same value for all elements.
    let rust_array = [3; 5];

    // Trying to access an index outside the
    // array's range will make the program
    // panic resulting in a runtime error.
}
