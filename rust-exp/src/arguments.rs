pub fn args() {
    // Positional arguments
    println!(
        "{0} am {1} {2}, {0} like {3}",
        "I", "Ranjith", "Sekar", "Programming"
    );

    // Named arguments
    println!(
        "{value1} is {value2}",
        value1 = "Rust programming",
        value2 = "fun."
    )
}
