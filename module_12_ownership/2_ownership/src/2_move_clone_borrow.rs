// Run with: cargo run --bin move_clone_borrow
//
// Moving, cloning, and borrowing data in light of ownership.

// Moving data transfers ownership from one variable to another.
fn ownership_move() {
    let x: String = String::from("hello");
    // this moves x to y
    let y: String = x;
    println!("{}", y);
    // Try to print x and run the code. What do you think will happen?
    // println!("{}", x);
}

// Cloning creates a new copy with its own new ownership.
// The original data is unaffected.
fn ownership_clone() {
    let x: String = String::from("hello");
    // this clones x to y
    let mut y: String = x.clone();
    // Try to modify y by adding more characters to it.
    // What do you think will happen?
    // What if we drop x? Would y be affected?
    // y.push_str(" everyone!");
    println!("{}", y);
    println!("{}", x);
}

// Borrowing creates a reference: no ownership transfer, no copy.
fn ownership_borrow() {
    let x: String = String::from("hello");
    // this borrows x
    let y: &String = &x;
    println!("{}", y);
    // We can drop the reference y without affecting the string,
    // since it is owned by x.
    drop(y);
    println!("{}", x);
}

fn main() {
    ownership_move();
    ownership_clone();
    ownership_borrow();
}
