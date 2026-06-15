// Run with: cargo run --bin borrow_checker
//
// This code does not compile (on purpose): Rust realizes that
// after creating ref0, but before using it, the vector is mutated
// using push, which causes dangerous behavior.
fn main() {
    let mut v: Vec<String> = vec![String::from("str1"), String::from("str2")];
    // reference to the first element.
    let ref0: &String = &v[0];

    // We are inserting many elements to the vector.
    // This causes the vector to resize and changes the location
    // of its elements in memory.
    for i in 0..10 {
        v.push(format!("str{}", i));
    }

    // Now, the old ptr address is no longer valid.
    println!("{}", ref0);
}
