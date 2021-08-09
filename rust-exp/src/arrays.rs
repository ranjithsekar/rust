pub fn run() {
    let mut numbers: [i32; 3] = [1, 2, 3];

    println!("{}", numbers[0]);

    numbers[2] = 40;

    // print all values
    println!("{:?}", numbers);

    // slice
    let sliced = &numbers[0..2];
    println!("{:?}", sliced);
}
