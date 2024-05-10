fn main() {
    while_loop();
    for_in_loop();
    print_even_num();
    less_than_greater_than();
}

// While Loop
fn while_loop() {
    let num_array: [i32; 9] = [1,2,3,4,5,6,7,8,9];

    let mut number: usize = 5;

    while number != 0  {
        println!("Looping through indexes {number} item in array {}", num_array[number]);
        number -= 1;
    }

    println!("While Loop Liftoff");
}

// For in Loop
fn for_in_loop() {
    let num_array: [i32; 9] = [1,2,3,4,5,6,7,8,9];

    for element in num_array{
        println!("for element {element} in {:?}",num_array);
    }

    println!("For in Loop Liftoff");
}

// print out even numbers btwn 0 -10

fn print_even_num() {
    let num_array: [i32; 9] = [1,2,3,4,5,6,7,8,9];

    for element in num_array{
        if element % 2 == 0 { println!("The even numbers are {}", element); };
    }

}

fn less_than_greater_than() {
    let num_array: [i32; 5] = [5, 10, 15, 20, 50];

    for element in num_array {
        if element < 10 { println!("The {element} is less than 10")} else if element > 10 { println!("The {element} is greater than 10")};
    }
}