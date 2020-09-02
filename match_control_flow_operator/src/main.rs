#[derive(Debug)]
enum Coin{
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    NewYork,
}

fn main() {
    println!("Hello, world!");

    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Nickle));
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Quarter(UsState::NewYork)));

    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    // let mut count = 0;

    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("coin state is {:#?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


// check more into if let syntax