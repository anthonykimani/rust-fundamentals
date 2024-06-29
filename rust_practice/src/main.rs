mod arrays;
mod functions_and_modules;


use arrays::{array::static_array,vector::dynamic_array};
use functions_and_modules::modules::{modules, name_helpers::get_full_name};

fn main() {
    // static_array();
    // dynamic_array();
    modules();
}
