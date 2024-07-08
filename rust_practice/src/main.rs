mod arrays;
mod functions_and_modules;
mod control_flows;
mod closures_and_anonymous_functions;
mod match_expressions;
mod generics_and_traits;
mod threading_basics;

use arrays::{array::static_array,vector::dynamic_array};
use functions_and_modules::modules::{modules, name_helpers::get_full_name};
use control_flows::conditionals::{test_if};
use closures_and_anonymous_functions::closures::{test_closures};
use match_expressions::match_expressions::{match_expressions};
use generics_and_traits::traits_file::{dog_functionality};
use threading_basics::threading::{main_thread, spawn_thread};

fn main() {
    // static_array();
    // dynamic_array();
    // modules();
    // test_if();
    // test_closures();
    // match_expressions();
    // dog_functionality();
    spawn_thread();
}
