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

    // convert to string
    let number_one: i32 = 101;
    let mut number_one_string: String = number_one.to_string();

    number_one_string.push_str("people in the room");
    println!("Hey {number_one_string}");

    let input_one: &str = "21";
    let input_number: i32 = input_one.parse().expect("was expecting an integer");

    println!("I'm finally {input_number}");

    // let _first_input: i32 = 200;
    // let _second_input:i16 = first_input as i16;

    // println!("Big cup to ")
    

    sum(3,5);

    // tuples in rust
    let (data, status, reason) = do_sum_maths(45,22);

    println!("Sum = {data} was it success {status} reason: {reason}");

    // enums in rust
    const age: i32 = 13;

    let ages_2: i32 = 15;

    #[derive(Debug)] // An Example of an attribute in rust
    enum STATUS {
        ADULT, CHILD
    }

    let user_status: STATUS = if age > 18 {
        STATUS::ADULT
    } else {
        STATUS::CHILD
    };

    println!("The User Status is {:?}", user_status);

}

// Functions in rust
fn sum(number_one: u8, number_two: u8) -> u8 {
    let sum: u8 = number_one + number_two;
    println!("The Sum of number_one and number_two is {}", sum);
    return sum;
}

fn do_sum_maths(par_1: i32, par_2: i32) -> (i32, bool, String) {
    return (0, false, "Failed for some reason".to_string())
}