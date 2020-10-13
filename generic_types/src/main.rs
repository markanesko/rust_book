struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MultiPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MultiPoint<T, U> {
    fn mixup<V, W>(self, other: MultiPoint<V, W>) -> MultiPoint<T, W> {
        MultiPoint {
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {
    println!("Hello, world!");

    let number_list = vec![23, 32, 22, 122, 2, 4432];

    // let result = largest(&number_list);

    // println!("largest number is {} ", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 3.3, y: 3.5 };

    let strange = MultiPoint { x: 5, y: 3.3 };
    let strange_same = MultiPoint { x: 4, y: 4 };

    println!("p.x = {}", integer.x());
    println!("distance is {}", float.distance_from_origin());

    let strange_mixup = strange.mixup(strange_same);

    println!("strange mixup is strange... {}", strange_mixup.x)

}


// fn largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for number in list{
//         if number > largest {
//             largest = number;
//         }
//     }

//     largest
// }



// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = list[0];

//     for item in list {
//         if item > largest{
//             largest = item;
//         }
//     }

//     largest
// }


fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
