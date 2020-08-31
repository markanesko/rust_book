// use std::io;
use rand::Rng; // Rng is trait
use std::cmp::Ordering;

fn main() {
    println!("guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 100);

    // println!("secret number is: {}", secret_number);
    println!("secret number is generated");

    loop {
        println!("input your guess to do so");

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("failed to read the line");

        // let guess: u32 = guess.trim().parse().expect("please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter number");
                continue;
            }
        };
        println!("you have guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you guessed right");
                break;
            }
        }
    }
}
