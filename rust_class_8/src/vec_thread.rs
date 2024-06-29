use std::thread;

pub fn vec_thread(){
    let v:Vec<i32> = vec![1,2,3,4];
    let handle = thread::spawn(move||{
        println!("Hello {:#?}", v)
    });

    handle.join().unwrap();
}