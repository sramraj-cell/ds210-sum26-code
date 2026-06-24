use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn no_synchronization() {
    // Shared counter between all threads
    let mut counter = 0;
    let address_of_counter = &mut counter as *mut i32 as usize;

    let timer = Instant::now();

    // Spawn 5 threads to count
    let mut threads = Vec::with_capacity(5);
    for i in 0..5 {
        // Count from 0 to 10_000_000 using pointer.
        let f = move || {
            println!("thread {i} started");
            let counter: *mut i32 = address_of_counter as *mut i32;
            for _ in 0..10_000_000 {
                unsafe {
                    *counter = *counter + 1;
                }
            }
            println!("thread {i} finished");
        };
        let thread = thread::spawn(f);
        threads.push(thread);
    }

    // Wait for all threads to complete
    for thread in threads {
        thread.join().unwrap();
    }

    println!("no synchronization: time taken is {:?}", timer.elapsed());
    println!("no synchronization: counter now is    {}", counter);
    println!("no synchronization: counter should be {}", 10_000_000 * 5);
}

fn with_mutex() {
    // Shared counter between all threads
    let counter = Mutex::new(0);
    let counter = Arc::new(counter);

    let timer = Instant::now();

    // Spawn 5 threads to count
    let mut threads = Vec::with_capacity(5);
    for i in 0..5 {
        let counter2 = counter.clone();
        // Count from 0 to 1_000_000 using mutex.
        let f = move || {
            println!("thread {i} started");
            for _ in 0..10_000_000 {
                let mut lock = counter2.lock().unwrap();
                let counter: &mut i32 = lock.deref_mut();
                *counter = *counter + 1;
            }
            println!("thread {i} finished");
        };
        let thread = thread::spawn(f);
        threads.push(thread);
    }

    // Wait for all threads to complete
    for thread in threads {
        thread.join().unwrap();
    }

    println!("no synchronization: time taken is {:?}", timer.elapsed());
    println!("with mutex: counter now is    {}", counter.lock().unwrap());
    println!("with mutex: counter should be {}", 10_000_000 * 5);
}

fn main() {
    no_synchronization();
    with_mutex();
}