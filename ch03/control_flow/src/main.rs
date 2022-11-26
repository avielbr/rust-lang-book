fn main() {
    let a: [u32; 5] = [0, 1, 2, 3, 4];
    let mut index: usize = 0;

    // This works...
    while index < (a.len() - 1) {
        println!("{}", a[index]);
        index += 1;
    }

    // This is better though...
    for item in a {
        println!("{}", item);
    }

    // Or alternatively...
    for number in 0..5 {
        println!("{}", number);
    }

    println!("done.");
}
