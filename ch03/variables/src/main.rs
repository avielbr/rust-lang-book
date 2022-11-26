// Variables and mutability

fn var_mut() {
    let mut x: u32 = 5;
    println!("The value of x is {x}");
    x = 6; // <-- This is allowed because x is mutable
    println!("The value of x is {x}");
}

// Constants
const THREE_HRS_IN_SECONDS: u32 = 60 * 60 * 3;

// Shadowing
fn shadow() {
    /*
    The 'let' keyword allows transformations to be made to an
    immutable variable, with those transformations being valid
    only within the current scope. Those changes dissappear
    once the scope is left.

    The variable remains immutable (x = 7 would not work).
    */
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("Inner scope x is {x}"); // Will print 12
    }
    
    println!("Outer scope x is {x}"); // Will print 6

    // ---

    /*
    The 'let' keyword effectively creates a new variable!
    That's why it's OK to reassign a variable, even with a
    different data type, using 'let'.
    */
    let spaces = "   "; // String type
    let spaces = spaces.len(); // Number type -- This is OK in Rust!
    println!("{}", spaces);
}

fn data_types() {
    // Just messing around with different data types...
    let x: u8 = 200;
    let y: i8 = 100;
    let z: usize = 1_000_000_000_000;

    let i: f32 = 2.6;
    let j: f64 = 2.5;

    let power = u32::pow(2, 8);
    assert_eq!(power, 2 << (8-1));
    
    println!("{power}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

//
fn main() {
    var_mut();
    println!("{THREE_HRS_IN_SECONDS}");
    shadow();
    data_types();
}

