// Run with: cargo run --bin with_references
//
// The same code as 1_with_pointers.rs, but with a reference
// instead of a pointer. This does not compile (on purpose):
// we cannot mutably borrow v in helper_function because v was
// already borrowed earlier when we created e0.
//
// Exercise: paste this code into Aquascope
// (https://cel.cs.brown.edu/aquascope/) and click `interpret`
// to see what would have gone wrong had Rust allowed it to run.
pub fn helper_function(v: &mut Vec<String>) {
    for i in 0..2 {
        v.push(format!("{}", i));
    }
}

pub fn main() {
    let mut v: Vec<String> = vec![String::from("hello"), String::from("bye")];
    let e0: &String = &v[0];

    helper_function(&mut v);

    println!("e0 refers to {}", *e0);
    println!("done");
}
