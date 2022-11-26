use std::io;

fn main() {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("failed to read.");

    println!("{}", slice_first_word(&s));
}

fn slice_first_word(s: &String) -> String {
    let mut buff = String::new();
    for letter in s.chars() {
        let curr: String = letter.to_string();
        if letter != ' ' {
            buff.push_str(&curr);
        }
        else {
            break;
        }
    }
    buff
}
