// Run with: cargo run --bin read_write_own
//
// Permissions over variables:
// - x has R (read) and O (ownership) permissions, but no W (write)
//   since it is not mutable.
// - y has R, W, and O permissions.
//
// Confirm this by pasting the code into Aquascope
// (https://cel.cs.brown.edu/aquascope/) and clicking `permissions`.
fn main() {
    let x: String = String::from("hello");
    let mut y: String = String::from("bye");
    println!("{}", x);
    println!("{}", y);
    drop(x);
    drop(y);
}
