// Run with: cargo run --bin memory_leak
//
// We use the Tracker from project 3 to track our strings: the
// tracker reports which values were never destructed.
//
// When the vector gets cleared, it only forgets the pointers.
// No one destructs or frees the strings we put on the heap:
// the tracker shows they are still alive. This is a memory leak!
use tracker::{Tracked, Tracker};

fn put_on_heap(x: Tracked<String>) -> *mut Tracked<String> {
    unsafe {
        // Allocate enough memory on the heap to store a tracked String,
        // then initialize that memory with x.
        let ptr: *mut Tracked<String> =
            libc::malloc(size_of::<Tracked<String>>()) as *mut Tracked<String>;
        std::ptr::write(ptr, x);
        ptr
    }
}

fn main() {
    let mut tracker = Tracker::new();

    let mut v: Vec<*mut Tracked<String>> = vec![
        put_on_heap(tracker.track(String::from("str1"))),
        put_on_heap(tracker.track(String::from("str2")))
    ];

    // ... use the vector ...
    unsafe {
        println!("{}", *v[0]);
        println!("{}", *v[1]);
    }

    // We are done with the vector: we clear it without deleting
    // the strings. The vector only frees its own heap allocation
    // (the one containing the pointers), the strings remain on the
    // heap forever even though no one can use them anymore!
    v.clear();

    // The tracker confirms the strings were never freed.
    tracker.print_status();
    println!("program done!");
}
