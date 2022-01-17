struct Point<T> {
    x: T,
    y: T,
}

struct PointAnother<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointAnother<T, U> {
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        PointAnother {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    let number_list = vec![20, 40, 10, 50, 100];
    let char_list = vec!['a', 'b', 'c', 'd', 'e'];
    let p1 = Point { x: 1, y: 5 };
    let p2 = Point { x: 1.0, y: 2.0 };
    let p3 = PointAnother { x: 1, y: 2.0 };
    let p4 = PointAnother { x: "Rust", y: "a" };
    let p5 = p3.mixup(p4);
    println!("{} {}", p5.x, p5.y);
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The lagest is {}", largest);
    // println!("The lagest is {}", largest_i32(number_list));
    // println!("The lagest is {}", largest(number_list));
    // println!("The lagest is {}", largest(char_list));
}

// fn largest_i32(list: Vec<i32>) -> i32 {
//     let mut largest = list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
