use std::io;

fn main() {
    reverse_with_range_operator();
    print_to_input();
    terminal_input();
}

fn reverse_with_range_operator() {
    for i in (0..50).rev() {
        println!("{}", i);
    }
}

fn print_to_input() {
    println!("Enter any number");
    let mut input = String::new();
    let n = io::stdin().read_line(&mut input).expect("Failed to read line");
    let number = input.trim().parse().expect("Please enter a number");

    for i in 0..number {
        println!("{}", i);
    }

    let mut i: i32 = 0;

    while i < number {
        println!("{}", i);
        i += 1;
    }
}

fn terminal_input() {
    println!("Hello my name is ChatGPT, what's is your age?");
    let mut input = String::new();
    let n = io::stdin().read_line(&mut input).expect("Failed to read line");
    let number: i32 = input.trim().parse().expect("Please enter a number");
    println!("Hello I'm {} years old", input);
}
