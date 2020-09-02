
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        // self.area() > other.area() // oh this is so wrong... xd
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle{
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let height1 = 50;
    let width1 = 30;

    println!("area of the rectangle is {} square pixels", area_first(width1, height1));


    let rect1 = (30, 50);

    println!("area of the rectangle is {} square pixels", tup_area(rect1));


    let rectangle = Rectangle{
        width: 30,
        height: 50,
    };

    println!("area of the rectangle is {} square pixels", struct_area(&rectangle));


    println!("these are the struct values: {:#?}", rectangle);

    println!("rect area is {}", rectangle.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


    println!("area of square with edge of 7 is {}", Rectangle::square(7).area());

}

fn area_first(width: u32, height: u32) -> u32 {
    width * height
}

fn tup_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn struct_area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}

// good practice is to always leave struct owner to be the function that created it