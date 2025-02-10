use std::thread;
use std::time::Duration;

fn main(){
    let handle = thread::spawn(||{
        for i in 1..5{
            println!("Thread: { }", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    handle.join().unwrap();
}