// Run with: cargo run --bin boxes
//
// Box: heap allocation with ownership. Stable addresses like
// put_on_heap, but with automatic cleanup and no use-after-free.
// The tracker proves the cleanup happens!
use tracker::{Tracked, Tracker};

fn main() {
    let mut tracker = Tracker::new();

    let mut v: Vec<Box<Tracked<String>>> = vec![
        Box::new(tracker.track(String::from("str1"))),
        Box::new(tracker.track(String::from("str2")))
    ];

    // Just like with put_on_heap, every string lives in its own
    // heap allocation that never moves, and the vector only stores
    // the addresses.
    println!("address of first string used to be {:p}", v[0]);

    for i in 0..10 {
        v.push(Box::new(tracker.track(format!("str{}", i))));
    }

    println!("address of first string is still   {:p}", v[0]);
    println!("{}", v[0]);

    // No manual clean up needed! When v gets destroyed here, it
    // destroys the boxes inside it, and every box automatically
    // destructs its tracked string and frees its heap allocation.
    drop(v);

    // The tracker confirms all the strings were freed automatically.
    tracker.print_status();
    println!("program done!");
}
