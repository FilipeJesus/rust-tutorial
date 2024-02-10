fn main() {
    // Rust is statically types but it will wusually try
    // and infer data types. When many types are possible
    // then we must assign a type to a variable, like below.
    let guess: u32 = "42".parse().expect("Not a member.");

    // Scalars are single values: integers, floating points,
    // booleans and characters

    // Integers
    // u32 is a 32 bit unsigned integer while i32 is signed.
    // available sizes are 8, 16, 32, 63, 128 and arch (uses 
    // the computer arhitecture, usually used for indexing).
    //
    // signed variants can store number from -(2^(n - 1)) to
    // 2^(n - 1) - 1 which unsigned can store numbers from 
    // 0 to 2^(n - 1).
    // Integer overflow is possible, compilers should warn
    // you of this and these warnings will be disabled when
    // releasing. To explicitly handle the possibility of
    // overflow, you can use these families of methods provided 
    // by the standard library for primitive numeric types:
    // - Wrap in all modes with the wrapping_* methods, such as wrapping_add.
    // - Return the None value if there is overflow with the checked_* methods.
    // - Return the value and a boolean indicating whether there was overflow
    //   with the overflowing_* methods.
    // - Saturate at the value’s minimum or maximum values with the saturating_*
    //   methods.

    // Floating-Point Types
    // Rust have 2 floating point types: f32 and f64. All floating
    // point types are signed. f32 has single-precision while f64
    // has double precision
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // Numeric Operations

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let produce = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1 as integer type is used
    // remainder
    let remainder = 43 % 5;

    // The character type
    // char literals are specified with single quotes while string literals
    // which use double quotes.
    // char type is 4 bytes and represents a unicode scalar value.
    let c = 'z';
    let z: char = 'Z'; // Explicit type annotation
    let heart_eyed_cat = '😻';

    // Compound types
    // Rust has two primitive compound types: tuples and arrays.

    // Tuple type
    // Grouping together a number of values with a variety of types into
    // one compound type. They have fixed lengths once decalred.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // To get values out of a tuple you can either use pattern matching
    // to destructure a tupple value or access the element directly using
    // a period followed by the index
    let (x,y,z) = tup;
    println!("{}", x == tup.0);
    println!("{}", y == tup.1);
    println!("{}", z == tup.2);

    // The tuple without any values has a special name, unit. This value
    // and its corresponding type are both written () and represent an
    // empty value or an empty return type. Expressions implicitly return
    // the unit value if they don’t return any other value.

    // The Array Type
    // Unlike a tuple every element needs to be of the same type, they still
    // have a fixed length. useful when you want data allocated on the stack
    // rather than the heap or when you want to ensure the fixed length.
    // An array is a single chunk of memory of a known, fixed size that can
    // be allocated on the stack.
    let a = [1, 2, 3, 4, 5];

    // You can define an arrays type and length. Below each element is a i32
    // and 5 indicates that the array has 5 elements.
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // You can also initialise an array which contains the same value for each
    // element by defining the value followed by a ';' and the length.
    let a = [3; 5];

    // You can access elements of an array using indexing, like this:
    let first = a[0];
    let second = a[1];

    // When accessing an index which is passed the length of an array, Rust will
    // raise a runtime error at teh point of using the invalid index. Rust
    // actually first checkes that the index provided is within the range of the
    // array length, then raises the error if not. This is an example of Rust's
    // memory safe principles.
}
