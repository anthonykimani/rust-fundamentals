mod counties;
mod plants;
mod structs;

use counties::counties_kenya;
use plants::{poisonous_plants, fruits_plants};
use rust_math::num::{sqrt, gcd};
use structs::struct_file;

fn main() {
    counties_kenya();
    poisonous_plants::poisonous_plants();
    fruits_plants::fruit_plants();

    let result = sqrt(4.0);
    let result2 = gcd(10, 50);
    println!("result {result}");
    println!("gcd is {result2}");

    struct_file::struct_func();
}
