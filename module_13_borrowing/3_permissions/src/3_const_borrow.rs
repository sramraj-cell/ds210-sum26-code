// Run with: cargo run --bin const_borrow
//
// Permissions after (const) borrowing:
// - x starts with R and O.
// - When we create r: *r gets R (no W: not a mutable reference,
//   and no O: the reference does not own the data). x keeps R but
//   loses O: we cannot destroy the data while r is active!
//   Try it: add drop(x) between defining r and printing it.
// - After we print r, the reference is no longer active: *r loses
//   all permissions and x regains O.
// - After x is printed and goes out of scope, x loses all
//   permissions as well.
//
// Confirm this with Aquascope's `permissions` view!
fn main() {
    let x: i32 = 10;

    let r: &i32 = &x;

    println!("{}", r);

    println!("{}", x);
}
