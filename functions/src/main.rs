fn main() {
    println!("Hello, world!");
    other_fn();
    another_fn(17);
    yet_another_fn(-4, 5);

    // statement
    let _y = 6;
    // unlike in c let assignment doesn't returne value

    let _x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    println!("best function eva returned: {}", five());

    plus_one(7)
}

fn other_fn() {
    println!("im not like the others");
}

fn another_fn(x: i32) {
    println!("i received: {}", x);
}

fn yet_another_fn(x: i32, y: u64) {
    println!("i have two values: {} {}", x, y);
}

fn five() -> u8 {
    5
}

fn plus_one(x: u128) -> () { // returns empty tuple
    x + 1;
}
// Function bodies are made up of a series of statements optionally ending in an expression.
// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resulting value. Let’s look at some examples.
