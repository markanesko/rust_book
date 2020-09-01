fn main() {
    
    let mut s = String::from("hello");

    s.push_str(", is it  you?");

    println!("{}", s); 

    let x = 5;
    let y = x;

    println!(" {} {} ", x, y);

    // deep copy

    let s1 = String::from("stringy");
    let s2 = s1.clone();

    println!("{} {}", s1, s2);

    let s = String::from("tempy");

    let x = 7;

    move_fn(s);
    copy_fn(x);

    // println!("{} {}", x, s);
    println!("{}", x);

    let s_fn = gives_fn();

    let _ = take_and_give(s_fn);


}

fn move_fn(some_string: String){
    println!("{}", some_string);
}

fn copy_fn(some_int: u32){
    println!("{}", some_int);
}

fn gives_fn() -> String {
    let some_string = String::from("empty");

    some_string
}

fn take_and_give(a_string: String) -> String {

    println!("{} in fn", a_string);

    a_string
}


// Ownership rules

// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

// String type
// String is allocated on heap 



// let s1 = String::from("data 1");
// let s2 = s1;

// println!("{} {}", s1, s1);
// leads to double free error! (freeing memory twice)

// in rust shallow copy with example above is called move

// drop is called whenever we exist scope 

// Rust will never automatically create “deep” copies of your data.