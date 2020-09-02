fn main() {
    println!("Hello, world!");

    let user1 = User {
        username: String::from("userinho"),
        email: String::from("email@email.com"),
        sing_in_count: 3,
        active: true,
    };

    println!("{:#?}", user1);

    let mut user2 = User{
        username: String::from("userinho"),
        email: String::from("email@email.com"),
        sing_in_count: 3,
        active: true,
    };

    user2.email = String::from("emayil2@mailoo.org");
    println!("{:#?}", user2);

    let user3 = User{
        email: String::from("konjic"),
        ..user1
    };
    println!("{:#?}", user3);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Color(0, 0, 0);

}

#[derive(Debug)]
struct User{
    username:       String,
    email:          String,
    sing_in_count:  u64,
    active:         bool,
}

fn build_user(email: String, username: String) -> User {

    // User {
    //     username: username,
    //     email: email,
    //     sing_in_count: 1,
    //     active: true,
    // }
    User {
        username,
        email,
        sing_in_count: 1,
        active: true,
    }

}


// Tuple struct, same as tuple but has name


// Struct cant store reference to, eg. String unless we include lifetime