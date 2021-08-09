pub fn var() {
    let mut name = "Ranjith";
    println!("My name is {}", name);
    name = "Ranjith Sekar";
    println!("My full name is {}", name);
    
    // Multiple variables
    let (name, age, interest) = ("Ranjith", "30", "Programming");
    println!("My name is {}, I am {} and I like {}", name, age, interest);
}
