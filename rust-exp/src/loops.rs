pub fn run() {
    let mut count = 0;
    loop {
        count += 1;
        println!("loop: {}", count);
        if count == 5 {
            println!(" I am 5");
            break;
        }
    }

    let mut num = 0;
    while num < 5 {
        num += 1;
        println!("while {}", num);
    }

    // For loop
    for x in 0..5 {
        println!("for: {}", x);
    }
}
