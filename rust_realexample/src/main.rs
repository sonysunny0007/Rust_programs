use std::sync::mpsc;
use std::thread;
use std::time::Duration;


fn main(){
    //create channel for communication
    let (pressure_tx, rx1)=mpsc::channel();
    let (temp_tx,rx2)=mpsc::channel();


    //Task1: Read pressure data
    let pressure_thread = thread::spawn(move ||{
        for i in 1..=5{
            let pressure = i*10;
            println!("Pressure task: { }", pressure);
            pressure_tx.send(pressure).unwrap();
            thread::sleep(Duration::from_secs(1));

        }
    });

    //Task2: Read temp data
    let temp_thread = thread::spawn(move ||{
        for i in 1..=5{
            let temp=i*10;
            println!("Temperature task: { }", temp);
            temp_tx.send(temp).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //Task 3: Process data
    let process_thread=thread::spawn(move ||{
        for _ in 1..=5{
            let pressure = rx1.recv().unwrap();
            let temperature = rx2.recv().unwrap();
            println!("Processing Data: Pressure={ }, Temperature={ }", pressure,temperature);
        }
    });

    //Wait for threads to complete
    pressure_thread.join().unwrap();
    temp_thread.join().unwrap();
    process_thread.join().unwrap();
}