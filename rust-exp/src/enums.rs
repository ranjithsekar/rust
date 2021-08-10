pub fn run() {
    get_quote(Day::MONDAY);
    get_quote(Day::SUNDAY);
}

fn get_quote(day: Day) {
    match day {
        Day::MONDAY => println!("Its Workday"),
        Day::SUNDAY => println!("Its Funday"),
    }
}

enum Day {
    MONDAY,
    // TUESDAY,
    // WEDNESDAY,
    // THURSDAY,
    // FRIDAY,
    // SATURDAY,
    SUNDAY,
}
