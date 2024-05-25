use std::io;
pub fn divide() {
    println!("Enter first number");
    let mut first_number = String::new();
    io::stdin().read_line(&mut first_number).expect("Failed to read line");
    let mut first_number_trim: i32 = first_number.trim().parse().expect("Please type a number");

        println!("Enter second number");
    let mut second_number = String::new();
    io::stdin().read_line(&mut second_number).expect("Failed to read line");
    let mut second_number_trim: i32 = second_number.trim().parse().expect("Please type a number");


    let divide = first_number_trim / second_number_trim;
    println!("{}", divide);
}