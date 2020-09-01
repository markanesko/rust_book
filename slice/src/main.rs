fn main() {
    println!("Hello, world!");

    let s = String::from("some long stringy");

    println!("{}", first_word_slice(&s));

    let s2 = String::from("other word");

    let word = first_word_all_slice(&s2[..]);
    println!("{}", word);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    println!("{:#?}", slice);

}


fn first_word_usize(s: &String) -> usize {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }

    s.len()

}

fn first_word_slice(s: &String) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]

}


fn first_word_all_slice(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]

}

// [starting_index..ending_index]