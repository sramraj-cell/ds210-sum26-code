// Run with: cargo run --bin drop_while_borrowed
//
// This does not compile (on purpose): it violates the third
// borrowing rule. We borrow v using e0, then, while the reference
// is still active, we destroy v using drop. The error says the
// code tries to *move* v (to drop) while v is *borrowed*.
//
// Exercise: use Aquascope (https://cel.cs.brown.edu/aquascope/)
// to find out what would have gone wrong had Rust let you run it.
pub fn main() {
    let v: Vec<i32> = vec![20, 30];
    let e0: &i32 = &v[0];

    drop(v);

    println!("e0 refers to {}", *e0);
    println!("done");
}
