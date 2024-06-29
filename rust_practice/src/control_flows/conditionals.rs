pub fn test_if(){
    let age_to_drive: u8 = 16u8;

    println!("Enter the Person's age");
    let user_input: &mut String = &mut String::from("");
    std::io::stdin().read_line(user_input).unwrap();

    let age: u8 = user_input.trim().parse::<u8>().unwrap();

    if age >= age_to_drive {
        println!("Issuing driver's license, because they are old enough");
    } else {
        println!("Not old enough to drive");
    }
}