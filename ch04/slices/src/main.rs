use std::io;

fn main() {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("failed to read.");

    println!("{}", slice_first_word(&s));
}

// My potentially naive implementation of https://doc.rust-lang.org/book/ch04-03-slices.html#:~:text=Here%E2%80%99s%20a%20small%20programming%20problem%3A%20write%20a%20function%20that%20takes%20a%20string%20of%20words%20separated%20by%20spaces%20and%20returns%20the%20first%20word%20it%20finds%20in%20that%20string.%20If%20the%20function%20doesn%E2%80%99t%20find%20a%20space%20in%20the%20string%2C%20the%20whole%20string%20must%20be%20one%20word%2C%20so%20the%20entire%20string%20should%20be%20returned.
// without using slices
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
