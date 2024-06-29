use std::thread;
use std::time::Duration;

pub fn thread_main() {
    let handle = thread::spawn(||{
        for _i in 0..20 {
            println!("Hello There");
            thread::sleep(Duration::from_millis(20));
        }
    });

    handle.join().unwrap(); // here we ask the thread to complete first then join the main thread
    println!("Hello Main Thread");
}