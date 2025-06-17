// // fn get_first_word(sentence: String) -> String {
// //     let mut ans = String::new();

// //     for word in sentence.chars() {
// //         if word == ' ' {
// //             break;
// //         }
// //         ans.push(word);
// //     }

// //     return ans;
// // }

// fn main() {
//     // // let name = String::from("shrit");
//     // // let char1 = name.chars().nth(0);

//     // // // match char1 {
//     // // //     Some(c) => println!("{}", c),
//     // // //     None => println!("No character found"),
//     // // // }

//     // // println!("{}", char1.unwrap());

//     // let sent = String::from("my name is Shrit");
//     // let first_word = get_first_word(sent);

//     // println!("{}", first_word);

//     // let mut arr = vec![1, 2, 3, 4, 5];

//     // for i in arr.iter() {
//     //     if *i == 3 {
//     //         // add 34 to the array
//     //         arr.push(34);
//     //     }
//     //     println!("{}", i);
//     // }

//     print!("hll");

//     // muatable

//     // let mut x = String::from("hello");

//     // for _ in 0..100 {
//     //     // will give errr if i don't add mut
//     //     x = x + "s";
//     // }
// }

// rust has it's own memory management system, it's like they have given developors cnrtl of how to manage memory, instead of something like javascript where there are garbage collectors which are not under our control.

// rust has this concept of ownership, which is like a pointer to the memory location of the variable., rust forces you to write a code in a way that you are not allowed to do crazy stuff like leaks and stuff

//that's why rust is fast, because it's not like javascript where there are garbage collectors which are not under our control.

// fn main() {
//     let x = 1; // crated on stack
//     let y = 3; // created on stack
//     println!("{}", sum(x, y));
//     println!("Hello, world!");
// }

// fn sum(a: i32, b: i32) -> i32 {
//     let c = a + b;
//     return c;

// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     // gives eror
//     println!("{}", s1); // Compiles now
// // }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = &s1;
//     println!("{}", s2);
//     println!("{}", s1);

//     // address

//     // length
//     println!("{}", s1.capacity());
//     println!("{}", s2.capacity());

//     // capacity
// }

// fn main() {
//     // borrowing

// }

// struct Rect {
//     width: u32,
//     height: u32,
// }

// // impl Rect {
// //     fn area(&self) -> u32 {
// //         return self.width * self.height;
// //     }
// //     fn perimeter(&self) -> u32 {
// //         return 2 * (self.width + self.height);
// //     }
// // }
// structs
// fn main() {
//     // struct User {
//     //     id: i32,
//     //     name: String,
//     //     email: String,
//     //     age: i32,
//     // }

//     // let mut user1 = User {
//     //     id: 1,
//     //     name: String::from("Shrit"),
//     //     email: String::from("shrit@gmail.com"),
//     //     age: 20,
//     // };

//     // user1.id = 2;

//     // println!("{}", user1.id);
//     // println!("{}", user1.name);
//     // println!("{}", user1.email);
//     // println!("{}", user1.age);

//     let rect1 = Rect {
//         width: 10,
//         height: 20,
//     };

//     println!("Area: {}", rect1.area());
//     println!("Perimeter: {}", rect1.perimeter());
// }

// enums
// enum Dir {
//     North,
//     South,
//     East,
//     West,
// }

// fn main() {
//     let dir = Dir::North;
// }

// pattern matching

// enum Shape {
//     Circle(f64),
//     Square(f64),
//     Rectangle(f64, f64),
// }

// fn main() {
//     let shape = Shape::Circle(10.0);
//     match shape {
//         Shape::Circle(radius) => println!("Circle with radius {}", radius),
//         Shape::Square(side) => println!("Square with side {}", side),
//         Shape::Rectangle(width, height) => {
//             println!("Rectangle with width {} and height {}", width, height)
//         }
//     }
// }

// // error management
// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let point = Point { x: 10, y: 20 };
//     println!("{}", point.x);

//     let point2 = Point { x: 10.0, y: 20.0 };
//     println!("{}", point2.x);
// }

// rust doesn't have a null : (

fn main() {
    let my_string = String::from("raman");
    match find_second_a(my_string) {
        Some(index) => println!("The letter 'a' is found at index: {}", index),
        None => println!("The letter 'a' is not found in the string."),
    }
}

fn find_second_a(s: String) -> Option<i32> {
    let mut count = 0;
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            count += 1;
            if count == 2 {
                return Some(index as i32);
            }
        }
    }
    return None;
}
