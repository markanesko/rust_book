fn main() {
    println!("Hello, world!");

    // panic!("now die");

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // println!("11th element is {}", v[10]);
    println!("10th element is {}", v[9]);


}


// panic! by default first clears the stack and then exits
// if performance is essential and small binary is required
// [profile.release]
// panic = 'abort'

// $ RUST_BACKTRACE=1 cargo run
// to see full stack trace