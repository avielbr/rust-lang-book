use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let ans = rand::thread_rng()
        .gen_range(1..=100);

    println!("game start.");
    println!("guess.");

    loop {
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("failed.");

        let guess: u32 = guess.trim().parse().expect("that's not a number.");

        println!("guessed: {guess}\n");

        match guess.cmp(&ans) {
            Ordering::Less      => println!("small."),
            Ordering::Greater   => println!("big."),
            Ordering::Equal     => {println!("fuck yes."); break;}
        }
    }
}
