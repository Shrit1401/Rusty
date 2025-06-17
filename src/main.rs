fn main() {
    // let name = String::from("shrit");
    // let char1 = name.chars().nth(0);

    // // match char1 {
    // //     Some(c) => println!("{}", c),
    // //     None => println!("No character found"),
    // // }

    // println!("{}", char1.unwrap());

    let sent = String::from("my name is Shrit");
    let first_word = get_first_word(sent);

    println!("{}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut ans = String::new();

    for word in sentence.chars() {
        if word == ' ' {
            break;
        }
        ans.push(word);
    }

    return ans;
}
