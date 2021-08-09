// Tuples are group of elements, max 12.
pub fn tup() {
    let days: (&str, &str, &str, i32) = ("Monday", "Tuesday", "Wednesday", 4);
    println!("{} | {} | {} | {}", days.0, days.1, days.2, days.3);
}
