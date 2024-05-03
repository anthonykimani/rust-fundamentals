fn main() {
    // signed integers in rust
    let number1: i8 = 10;
    let _number2: i16 = -20;
    let _number3: i32 = 30;
    let _number4: i64 =  -40;
    let _number5: i128 = 50;
    println!("The number is {}", number1);

    // unsigned integers in rust
    let unsigned_int: u8 = 10;
    let _unsigned_int_2: u16 = 200;
    let _unsigned_int_3: u32 = 3000;
    println!("The unsigned_int is {}", unsigned_int);

    // characters in rust
    let character_one: char = '🧪';
    println!("An example of a Testube emoji is {}", character_one);

    // floats in rust
    let _float_one: f32 = 0.032;



    // String in Rust
    // 1.str
    const my_age : &str = "He was 26yrs old";
    println!("The Teacher said {}", my_age );

    // 2. String
    let mut text_1: String = String::new();
    text_1.push_str("Wolf");
    println!("The Boy cried {}", text_1);

    let text_2: String = String::from("Rust");
    println!("We are learning {} for Blockchain Development", text_2);
    

    sum(3,5);

}

// Functions in rust
fn sum(number_one: u8, number_two: u8) -> u8 {
    let sum: u8 = number_one + number_two;
    println!("The Sum of number_one and number_two is {}", sum);
    return sum;
}