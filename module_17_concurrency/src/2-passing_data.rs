use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn function1(count: usize) {
    println!("In function 1");
    for i in 0..count {
        println!("ping");
    }
    println!("function 1 done");
}

fn main() {
    let count = 10;

    // The || { ... } syntax represents a "closure" that takes 0 parameters.
    // (see https://doc.rust-lang.org/book/ch13-01-closures.html)
    // The closure can use variables from the scope it is defined in (called "capturing").
    // In this case, it captures count by move.
    let f1 = move || {
        println!("In closure f1");
        function1(count);
    };

    println!("Right before spawn");
    let thread1 = thread::spawn(f1);

    // Below is an alternative but equivalent syntax.
    // let thread1 = thread::spawn(move || { function1(count) });

    // main is now waiting for thread.
    thread1.join().unwrap();
}
