// Run with: cargo run --bin use_after_free
//
// This code exhibits a use-after-free: undefined behavior!
// It may crash, print garbage, or appear to work fine.
fn put_on_heap(x: String) -> *mut String {
    unsafe {
        let ptr: *mut String = libc::malloc(size_of::<String>()) as *mut String;
        std::ptr::write(ptr, x);
        ptr
    }
}

fn main() {
    let v: Vec<*mut String> = vec![
        put_on_heap(String::from("str1")),
        put_on_heap(String::from("str2"))
    ];

    // Somewhere in the program, we decide we are done with the
    // first string, and destroy it.
    unsafe {
        std::ptr::read(v[0]);  // destructs the string.
    }

    // ... later, a different part of the program tries to use it!
    let ptr0: *mut String = v[0];
    unsafe {
        println!("{}", *ptr0);
        println!("program done!");
    }
}
