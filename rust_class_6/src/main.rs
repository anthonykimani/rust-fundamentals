use std;
use rand::Rng;

fn main() {
    // let _vector:Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
    // println!("{:?}, {:?}, {:?}", _vector[6], _vector[7], _vector[8]);
    // traffic_light(TrafficState::Red);
    // occurence("the lazy fox jumped the bridge lazy", "lazy")
    // print_desc();
    get_tuple(vec![(3,2), (2,2)]);

    get_tuple(vec![(3,5), (2,7)]);
}

enum TrafficState {
    Red,
    Orange,
    Green
}

struct TrafficStruct {
    red:String,
    orange: String,
    green: String,
}

fn traffic_light(status: TrafficState) {
    match status {
        TrafficState::Red => println!("STOP"),
        TrafficState::Orange => println!("GET READY"),
        TrafficState::Green => println!("GO"),
        _ => println!("TrafficLight is unknown")
    }
}

fn traffic_light_2(){}

// a string - the lazy fox jumped the bridge

fn occurence(text: &str, character: &str) {
    let mut character_count = 0;
    let text_vector: Vec<&str> = text.rsplit(' ').collect();
    for i in text_vector {
        let mut letter: &str = i;
        match character {
            letter => character_count+=1,
            _ => println!("Character not Found"),
        }
    }
    println!()
}


// create a program that uses the rand crate , that generates 1 to 10 and prints the numbers in descending order
fn print_desc() {
    let mut range = rand::thread_rng();
    let mut random_vector: Vec<u32> = vec![];
    for _i in 1..=10 {
        let random_number: u32 = range.gen_range(0..=10);
        random_vector.push(random_number);
    }
    random_vector.sort();
    for i in random_vector {
        print!("{}",i);
    }
}

// Write a Rust function that takes a vector of tuples (i32, i32)
// and returns the sum of all the first elements if the second elements are all even,
// the sum of all the second elements if the first elements are all odd, and 0 otherwise.

fn get_tuple(tuple_vector: Vec<(i32, i32)>) -> i32 {
    if(tuple_vector[0].1 % 2 == 0 && tuple_vector[1].1 % 2 == 0) {
        println!("{:?}", tuple_vector[0].0 + tuple_vector[1].0);
        return tuple_vector[0].0 + tuple_vector[1].0;
    } else if (tuple_vector[0].0 % 2 != 0  && tuple_vector[0].1 % 2 != 0) {
        println!("{:?}", tuple_vector[0].1 + tuple_vector[1].1);
        return  tuple_vector[0].1 + tuple_vector[1].1;
    } else {
        println!("0");
        return 0;
    }


    // let first_tuple = tuple_vector[1].0 % 2 == 0 && tuple_vector[1].1 % 2 == 0;
    // let second_tuple = tuple_vector[0].0 % 2 != 0  && tuple_vector[0].1 % 2 != 0;
    // match tuple_vector {
    //     _first_tuple => println!("hello"),
    //     _second_tuple => println!("bye"),
    //     _ => println!("Value is 0"),
    // }
}

fn sum_parameter(parameter_one: i32, parameter_two: i32) -> i32 {
    return parameter_one + parameter_two ;
}