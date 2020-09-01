fn main() {
    let s1 = String::from("long boi");

    let len = calculate_length(&s1);

    println!("{} with len {}", s1, len);

    let mut mut_s = String::from("longeer boi");
    change(&mut mut_s);

    println!("{}", mut_s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", longer boi");
}

// But mutable references have one big restriction:
// you can have only one mutable reference to a particular piece of data in a particular scope.
// let mut s = String::from("hello");

// let r1 = &mut s;
// let r2 = &mut s;

// We also cannot have a mutable reference while we have an immutable one.
// let mut s = String::from("hello");

// let r1 = &s; // no problem
// let r2 = &s; // no problem
// let r3 = &mut s; // BIG PROBLEM

// Note that a reference’s scope starts from where it is introduced and
// continues through the last time that reference is used.

// Rules of References


// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.
