use std::fmt::format;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn sync_main(){
    let (tx, rx) = mpsc::channel();

    for _i in 0..10{
        let tx_clone = tx.clone();
        thread::spawn(move||{
            tx_clone.send(format!("loop number {_i}"));
            thread::sleep(Duration::from_millis(10));
        });
    }

    for receiver in rx{
        println!("{}", receiver);
    }

}