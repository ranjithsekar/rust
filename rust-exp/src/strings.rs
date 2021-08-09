pub fn str() {
    let mut name = String::from("Ranjith ");
    name.push('S');
    name.push_str("ekar");
    println!("My name is: {}", name);

    let interest = "Programming";
    println!("My interest: {}", interest);

    // String functions
    println!("length: {}", name.len());
    println!("empty: {}", name.is_empty());
    println!("contains {}", name.contains("Sekar"));

    // Loop
    for word in name.split_whitespace() {
        println!("{}", word);
    }
    let langs = "Java,Python,Javascript,Angular,Rust";
    for lan in langs.split(",") {
        println!("{}", lan);
    }
}
