fn main() {
    println!("Hello, world!");

    // let mut s = String::new(); // new empty mut string s

    let data = "initial contents";
    let s = data.to_string(); // s created with content
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // strings are UTF-8 encoded

    let mut s = String::from("foo");
    s.push_str("bar");
    let s2 = "bar".to_string();

    s.push_str(&s2);

    println!("s2 still holds value {}", s2);

    s.push('l');

    println!("push method takes only one character {}", s);

    let s1 = String::from("first string, ");
    let s2 = String::from("second string");

    let s3 = s1 + &s2;

    println!("concatenation is {}", s3);

    // s1 is unusable now and we used & with s2 is because
    // + is using fn add(self, s: &str) -> String
    // also, String is translated to str because deref coercion
    // it turns &s2 into &s2[..]
    // also add doesn't take ownership from s2 and it is still valid
    // add takes ownership from s1 and appends copy of s2 on it
    // thus creating new string from s1 and values of s2
    // and by that ownership is returned to s3

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("s is {}", s);

    // format works same as println! but it returns new string
    // rather then printing it on screen

    // let hs = String::from("hello");
    // let h = hs[0];
    // error[E0277]: the type `std::string::String` cannot be indexed by `{integer}`

    // since rust works with UTF-8 encoded strings, and String is wrapper over Vec<u8> it is not always
    // possible to now what is the element at the given index
    // because all of this rust disables to even index strings

    // also indexing strings is operation considered to be in O(1)
    // and because UTF-8 it is not possible to guarantee that


    let hello = "Здравствуйте";

    let sub = &hello[0..4]; // first 4 bytes and it writes only Zd

    println!(" sub is {} ", sub);

    // println!("sub is {}", &hello[0..1]); // code panics

    for c in "नमस्ते".chars() { // iterate through chars
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() { // iterate through bytes
        println!("{}", b);
    }

    // Getting grapheme clusters from strings is complex, so this functionality is not provided by the standard library.
    



}

// strings are implemented as a collection of bytes and useful methods to go with them
// rust has only one string type and that is string slice str
// String type is in rusts standard library and not in core of the language

