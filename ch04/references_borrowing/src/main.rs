fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // Passing a -reference-, not lending!
    // let len = calculate_length(s1); This wouldn't work!
    
    println!("The length of '{}' is {}.", s1, len); // This wouldn't work without a reference
}

fn calculate_length(s: &String) -> usize { // Note the type is &String, not String
    s.len()
    // No need to transfer back ownership here -- because this function
    // never owned s1 to begin with!
}