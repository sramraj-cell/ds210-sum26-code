// Run with: cargo run --bin borrow_of_mut_data
//
// x is defined with mut, so it starts with R, W, and O.
// But r is a regular (const) reference to mutable data!
//
// Questions (use Aquascope to find the answers and try to
// understand why -- refer to the borrowing rules for help):
// - Do you think *r would have W permissions?
// - When we create r, x becomes actively borrowed! Does x lose
//   any permissions? If so, which ones and why?
fn main() {
    let mut x: i32 = 10;

    let r: &i32 = &x;

    println!("{}", r);

    println!("{}", x);
}
