fn main() {
    let s = String::from("hello");

    let s = takes_ownership(s);

    let x = 5;

    makes_copy(x); // Since u32 implements Copy...
    println!("{}", x);   // I can do this even after the function is called...
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    some_string // Transfers ownership
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
