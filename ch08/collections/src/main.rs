use std::collections::HashMap;

fn main() {
    // Vector stored on heap with variable length
    let mut v: Vec<i32> = Vec::new();
    for i in 0..=100 {
        v.push(i);
    }
    //println!("{:?}", v); // [0, 1, 2, ..., 100]

    // Vector initialized with values
    let v2 = vec![0, 1, 2];
    // println!("{:?}", v2);

    // Vector initialized with 100 zeroes
    let v3 = vec![0; 100];
    // println!("{:?}", v3);

    let violet = String::from("violet");
    let teal = String::from("teal");

    let mut scores = HashMap::new();

    // Note that this transfers owndership to the scores hashmap!
    scores.insert(violet, 10);
    scores.insert(teal, 50);

    // Update existing pair...
    scores.insert(String::from("teal"), 54);

    // Update or insert if doesn't exist...
    scores.entry(String::from("indigo")).or_insert(29);

    println!("{:?}", scores);
}
