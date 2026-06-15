// Run with: cargo run --bin mut_borrow
//
// Permissions after mutable borrowing:
// - x starts with R, W, and O.
// - When we create r: *r gets R and W (but obviously not O).
//   At the same time, x loses R (no mixing mut and const borrows),
//   W (only one active mutable borrow at a time), and O (cannot
//   destroy x while r is active).
// - *r loses all permissions after r expires, and x regains
//   R, W, and O.
// - x loses all permissions after it is done.
//
// Exercise: confirm this using Aquascope!
fn main() {
    let mut x: i32 = 10;

    let r: &mut i32 = &mut x;

    println!("{}", r);

    println!("{}", x);
}
