use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::fs;

fn main() {
    println!("Hello, world!");

    // panic!("now die");

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // println!("11th element is {}", v[10]);
    println!("10th element is {}", v[9]);

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem with creating file {:?}", e),
            },
            other_error => {
                panic!("problem with opening file: {:?}", other_error)
            },
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    }); // revisit this part after learning about closures

    let uf = File::open("hello.txt").unwrap();
    // If the Result value is the Ok variant, unwrap will return the value inside the Ok. 
    // If the Result is the Err variant, unwrap will call the panic! macro for us.

    let ef = File::open("hello2.txt").expect("failed to find hello2.txt");



}

fn read_username_from_file() -> Result<String, io::Error> {

    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    } // ; on the end wouldn't return values!!!
}

fn read_username_from_file_with_questionmark_propagator() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

    // The ? placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result values in Listing 9-6.
    // If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. 
    // If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.

}


fn read_username_from_file_questionmark_chained() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
    // Reading a file into a string is a fairly common operation, so Rust provides the convenient fs::read_to_string function that opens the file,
    // creates a new String, reads the contents of the file, puts the contents into that String, and returns it.

}


// panic! by default first clears the stack and then exits
// if performance is essential and small binary is required
// [profile.release]
// panic = 'abort'

// $ RUST_BACKTRACE=1 cargo run
// to see full stack trace

// recoverable errors
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }


// bad state is one of the following:

    // The bad state is not something that’s expected to happen occasionally.
    // Your code after this point needs to rely on not being in this bad state.
    // There’s not a good way to encode this information in the types you use.
