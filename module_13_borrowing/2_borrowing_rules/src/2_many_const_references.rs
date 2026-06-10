// Run with: cargo run --bin many_const_references
//
// Rust allows many const references at the same time: since none
// of them can modify the data, they are all safe. But it does not
// allow more than one active mutable reference at a time.
fn main() {
    let mut x1: i32 = 10;
    let r1: &i32 = &x1;
    let r2: &i32 = &x1;
    println!("r1 refers to {}", r1);
    println!("r2 refers to {}", r2);
    // This code runs because all references are const.
    // Change one or both references to a mut reference
    // and see what happens!
    // e.g.,
    // let r1: &mut i32 = &mut x1;
}
