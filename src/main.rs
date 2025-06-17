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

fn main() {
    // borrowing
}
