fn main() {
    let x: u32 = five();
    let x = plus_one(x);

    println!("{x}");
}

// This is a valid function in Rust
fn five() -> u32 {
    5
}

fn plus_one(x: u32) -> u32 {
    x + 1
}