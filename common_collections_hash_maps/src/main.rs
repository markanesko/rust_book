use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 23);

    let teams = vec![String::from("blue"), String::from("red")];
    let initial_scores = vec![10, 44];

    let mut scores: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    
    // For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, 
    // the values will be moved and the hash map will be the owner of those values,

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("field name is {}", field_name);

    // If we insert references to values into the hash map, the values won’t be moved into the hash map. The values that 
    // the references point to must be valid for at least as long as the hash map is valid. 

    let team_name = String::from("blue");
    let score = scores.get(&team_name);

    println!("score for {} is {:?} ", team_name, score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("orange"), 12);

    println!("{:?}", scores);

    // overwriting value
    scores.insert(String::from("blue"), 87);

    println!("{:?}", scores);

    scores.entry(String::from("blue")).or_insert(37);
    scores.entry(String::from("brown")).or_insert(37);
    
    println!("{:?}", scores);

    let text_for_counting = "hello hello some large some string world is wast, world is huge";

    let mut text_map = HashMap::new();

    for word in text_for_counting.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", text_map);

    // The or_insert method actually returns a mutable reference (&mut V) to the value for this key.
    // Here we store that mutable reference in the count variable, so in order to assign to that value, we must first dereference count using the asterisk (*). 
    // The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.

    // Rust uses slower and more secure hashing function which can be changed any time by implementing BuildHasher trait
    

}

// HashMap<K, V> stores a mapping of keys of type K to values of type V
