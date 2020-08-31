fn main() {
    println!("Hello, world!");

    // let x = 13;
    let mut x = 13;

    println!("value in x is: {}", x);

    x = 13;

    println!("value in x, after reassigning is: {}", x);

    const MAX_POINTS: u32 = 100_000;

    println!("constant value is: {}", MAX_POINTS);

    // shadowing concept in rust

    let y = 17;

    let y = y + 12;

    let y = y + 8;

    println!("y value is: {}", y);

    // shadowed variable can change its type

    let spaces = "    ";
    let spaces = spaces.len();

    println!("spaces: {}", spaces);

    // let mut spaces = "   ";
    // spaces = spaces.len();

    // addition
    let sum = 5 + 10;
    println!("sum: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient: {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {}", remainder);

    let t = true;

    let f: bool = false;

    println!("t and f: {} {}", t, f);

    let c = 'Z';
    let z = 'z';
    let heart_eyed_cat = '😻';

    println!("c z and hec: {} {} {}", c, z, heart_eyed_cat);

    // compound types in rust are types that can save multiple values into one variable
    // two primitive compound types are
    // tuple and array
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    let tup: (i32, f32, bool) = (500, 22.3, true);

    // println!("whole tuple is: {}", tup);
    println!("whole tuple is: {:#?}", tup);

    let (x, y, z) = tup;

    println!("x, y, z: {}, {}, {}", x, y, z);

    let first = tup.0;

    let second = tup.1;

    let third = tup.2;

    println!("{}, {}, {}", first, second, third);

    // the array type
    // arrays have fixed length like tuples

    let arr = [1, 2, 2, 4, 3];

    println!("arr: {:#?}", arr);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("months: {:#?}", months);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("a: {:#?}", a);

    let a = [13; 15];

    println!("a: {:#?}", a);

    println!("first and third are => {} {}", a[0], a[2]);

    // println!("index faild => {}", a[a.len() + 1]);
}

// differences betwwen variables and constants

// First, you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable.
// With constants you must always annotate the type.
// Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.
// The last difference is that constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.

// rust has scalar and compund types
// Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
// 8-bit	i8	u8
// 16-bit	i16	u16
// 32-bit	i32	u32
// 64-bit	i64	u64
// 128-bit	i128	u128
// arch	    isize	usize

// rusts char type is four bytes in size and such as that it represents the unicode scalar value
