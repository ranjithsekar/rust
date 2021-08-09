pub fn cond() {
    let attendance = 75;
    let fees_paid: bool = true;

    if attendance >= 80 && fees_paid {
        println!("you can write exam!");
    } else {
        println!("Meet principal");
    }
}
