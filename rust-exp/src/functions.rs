fn greeting(greet: &str, name: &str) {
    println!("Good {}!! Mr. {}", greet, name);
}

fn add(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

fn closure() {
    let n3 = 10;
    let add = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("sum is: {}", add(2, 2));
}

pub fn run() {
    greeting("Morning", "Ranjith");
    println!("Sum is: {}", add(4, 4));
    closure();
}
