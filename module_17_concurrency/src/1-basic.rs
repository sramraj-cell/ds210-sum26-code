use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn function1() {
    println!("In function 1");
    for i in 0..10000 {
        println!("ping");
    }
    println!("function 1 done");
}

fn function2() {
    println!("In function 2");
    for i in 0..10000 {
        println!("pong");
    }
    println!("function 2 done");
}

// Look at the output with JOIN and SLEEP set to false
// You will notice the functions do not finish executing before the program terminates!
// Enable SLEEP and observe the output.
// Same with JOIN.
const JOIN: bool = true;
const SLEEP: bool = false;

fn main() {
    let thread1 = thread::spawn(function1);
    let thread2 = thread::spawn(function2);

    if JOIN {
        thread1.join().unwrap();
        thread2.join().unwrap();
    }
    if SLEEP {
        sleep(Duration::from_secs(1));
    }
}