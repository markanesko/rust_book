use std::fmt::Display;

#[derive(Debug)]
struct ImportantDingus<'a> {
    part: &'a str,
}

fn main() {
    println!("Hello, world!");

    let string1 = String::from("long string is fuck all long");

    {
        let string2 = String::from("shorty");
        let result = longest(string1.as_str(), string2.as_str());
        println!("longest is {}", result)
    }

    let texty = String::from("Call me call me. My name is blondie...");
    let first_sentence = texty.split('.').next().expect("danger danger");

    let i = ImportantDingus{
        part: first_sentence,
    };

    println!("important something {:?} ", i)

}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {

    if x.len() > y.len() {      // this is funky because x and y come from another scope
        x                       // and since that they die in original scope when they enter this one
    } else {                    // comment above is for situation without lifetime
        y
    }
}


fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("announcement {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}