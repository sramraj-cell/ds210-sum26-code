// Run with: cargo run --bin put_on_heap
//
// Fixes the dangling pointer by storing every string in its own
// heap allocation, and only storing pointers in the vector.
fn put_on_heap(x: String) -> *mut String {
    unsafe {
        // Allocate enough memory on the heap to store a String,
        // then initialize that memory with x.
        let ptr: *mut String = libc::malloc(size_of::<String>()) as *mut String;
        std::ptr::write(ptr, x);
        ptr
    }
}

fn main() {
    let mut v: Vec<*mut String> = vec![
        put_on_heap(String::from("str1")),
        put_on_heap(String::from("str2"))
    ];

    // pointer to the first element.
    let ptr0: *mut String = v[0];

    // We are inserting many elements to the vector, like before.
    // The vector still resizes, and its elements still move in memory.
    // But the elements are now just pointers (i.e., addresses)!
    // The strings themselves never move from their own heap allocations.
    for i in 0..10 {
        v.push(put_on_heap(format!("str{}", i)));
    }

    println!("address of first string used to be {:p}", ptr0);
    println!("address of first string is still   {:p}", v[0]);

    unsafe {
        println!("dereferencing the pointer");
        println!("{}", *ptr0);
        println!("program done!");
    }
}
