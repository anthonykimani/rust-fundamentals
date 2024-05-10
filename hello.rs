// use std::mem;

// fn analyse_slice(slice: &[u32; 5]){
//     println!("The first element is {}", slice[0]);
//     println!("The Last element is {}", slice.len());
// }

fn main() {
    // let x: i16 = 130; //i32 is an integer type
    // let _y: i32;
    // let emoji = '😊';

    // assert_eq!(x, 130);
    // println!("success!");
    // println!("{}", emoji);
    // println!("1-2={}", 1i32 - 2);

    // // tuples
    // let tuple_example = (2i16, 4i16, 8i16);
    // let confused_example = ("hello", 3u8, 4i16);

    // println!("{:?}", tuple_example);

    // let ( _a, _b, _c) = confused_example;
    // println!("These are the destructured tuple elements: {:?}, {:?}, {:?}", _a, _b, _c);

    // //Arrays
    // let _xs:[u32; 5] = [1,2,3,4,5];
    // println!("{:?}", _xs[2]);

    // // Arrays are stack allocated.
    // println!("Array occupies {} bytes", mem::size_of_val(&_xs));

    // //print number of element in Array
    // println!("{:?}", _xs.len());

    // analyse_slice(&_xs);

    // // custom Types

    // // 1. Unit Structs
    // struct Person {
    //     name: String,
    //     age: u8,
    // }

    // // 2. Tuple Structs
    // struct Pair(i32, u8);


    // let name = String::from("Anto");
    // let age = 32;
    // let anto = Person { name, age };
    // println!("{:?}", anto);

    let number1: i8 = 10;
    let number2: i16 = 20;
    let number3: i32 = 30;
    let number4: i64 =  40;
    let number5: i128 = 50;
    println!("The number is {}", number1);
    println!("The second number is {}", number2);
}