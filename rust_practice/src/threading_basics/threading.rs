use std::thread::{spawn};

pub fn main_thread() {
    let mut x: u128 = 0u128;
    println!("Starting Main Thread");
    for i in 1..50_000_000_000 {
        x += i;
    }
    println!("Finished Main Thread");
}

pub fn spawn_thread() {

    let thread_fn = || {
        println!("Starting CPU Thread");
        let mut x: u128 = 0u128;
        for i in 1..50_000_000_000 {
            x += i;
        }
        println!("Finished CPU Thread");
    };
    let handle_thread_one = spawn(thread_fn);
    let handle_thread_two = spawn(thread_fn);
    let handle_thread_three = spawn(thread_fn);
    main_thread();
    handle_thread_one.join();
    handle_thread_two.join();
    handle_thread_three.join();

}