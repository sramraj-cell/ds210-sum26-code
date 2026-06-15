// Run with: cargo run --bin manual_cleanup
//
// To avoid the memory leak, we must remember to clean up after
// ourselves: for every string, we must destruct it (with
// std::ptr::read) and free the memory we allocated for it with
// malloc (with libc::free). The tracker confirms it worked!
use tracker::{Tracked, Tracker};

fn put_on_heap(x: Tracked<String>) -> *mut Tracked<String> {
    unsafe {
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

    // We are done with the vector: we must manually delete every element.
    unsafe {
        std::ptr::read(v[0]);                   // destructs the string.
        libc::free(v[0] as *mut libc::c_void);  // frees the malloc-ed memory.
        std::ptr::read(v[1]);                   // destructs the string.
        libc::free(v[1] as *mut libc::c_void);  // frees the malloc-ed memory.
    }

    // The pointers in the vector are now all dangling!
    // We clear the vector so that no one can use them by accident.
    v.clear();

    // The tracker confirms that all the strings were freed.
    tracker.print_status();
    println!("program done!");
}
