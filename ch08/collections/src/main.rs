fn main() {
    // Vector stored on heap with variable length
    let mut v: Vec<i32> = Vec::new();
    for i in 0..=100 {
        v.push(i);
    }
    //println!("{:?}", v); // [0, 1, 2, ..., 100]

    let third = &mut v[2];
    *third = 6;
    println!("{third}");
    println!("{}", v[2]);

    // Vector initialized with values
    let v2 = vec![0, 1, 2];
    // println!("{:?}", v2);

    // Vector initialized with 100 zeroes
    let v3 = vec![0; 100];
    // println!("{:?}", v3);

}
