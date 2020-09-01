fn main() {
    println!("Hello, world!");

    let x = 13;

    if x > 42 {
        println!("this is neat");
    } else {
        println!("not so much");
    }

    // value of the condition must be bool!!!
    
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if is an expression
    let condition = true;
    let bumber = if condition { 5 } else { 6 };

    println!("value of bumber is: {}", bumber);


    loop {
        println!("chuckles");
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 11 {
            break counter * 12;
        }
    };
    println!("The result is {}", result);

    let mut num = 3;

    while num != 0 {
        println!("{}!", num);

        num -= 1;
    }

    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("arr of {} is {}", index, arr[index]);

        index += 1;
    }

    // or

    for element in arr.iter() {
        println!("element is {}", element);
    }

    for number in (1..17).rev() {
        println!("{}", number);
    }

}



// Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
