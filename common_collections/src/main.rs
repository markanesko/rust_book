fn main() {
    println!("Hello, world!");

    let v: Vec<i32> = Vec::new();
    
    let mut v = vec![1, 2, 3];

    v.push(6);

    let vect = vec![1, 2, 3, 4, 5];

    let third: &i32 = &vect[2];

    println!("third element is {}", third);

    match v.get(2) {
        Some(third) => println!("third element is {} ", third),
        None => println!("there is no third element"),
    }
    
    let bad_vector = vec![1, 2, 3, 4, 5, 6];

    // let does_not_exists = &bad_vector[100]; // this one panics 
    let does_not_exists = bad_vector.get(100);  // this one returns None
    

    let mut mutabl = vec![1, 2, 3, 4, 5];

    let first = &mutabl[0];

    // mutabl.push(123); // cant do this because it is already borrowed as immutable, adding element might create need for memory reallocating and it could leave variable with bad pointer

    println!("let see what is in there {}", first);

    
    let v = vec![100, 32, 57];
    for i in &v { // get immutable references to each element in a vector
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // if there is need for storing elements with different types we can use enum
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("stringy")),
        SpreadsheetCell::Float(12.22),
    ];

    println!("row is {:#?}", row);

    let row2 = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("stringy")),
        SpreadsheetCell::Float(12.22),
    ];

    let table = vec![row, row2];

    println!("table is {:#?}", table);

}

// vectors can only store values of same type
// vectors are implemented using generics
// like any other struct, a vector is freed when it goes out of scope
