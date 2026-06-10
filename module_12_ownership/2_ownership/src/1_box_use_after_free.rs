// Run with: cargo run --bin box_use_after_free
//
// This code does not compile (on purpose): the compiler catches
// the use-after-free that raw pointers happily allowed.
fn main() {
    let b: Box<String> = Box::new(String::from("str1"));
    println!("{}", b);

    // This destroys the box, which destructs its string and
    // frees the heap memory.
    drop(b);

    // The compiler knows b was destroyed and will not let us use it!
    println!("{}", b);
}
