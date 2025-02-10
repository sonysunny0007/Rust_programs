use std::thread;
use std::time::Duration;

fn task1(){
    println!("Running task1");
    thread::sleep(Duration::from_millis(500));
    println!("Task1 Done!");
}

fn task2(){
    println!("Running task2");
    thread::sleep(Duration::from_millis(500));
    println!("Task2 Done!");
}

fn main(){
    let handle1=thread::spawn(||{
        task1();
    });

    let handle2=thread::spawn(||{
        task2();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}