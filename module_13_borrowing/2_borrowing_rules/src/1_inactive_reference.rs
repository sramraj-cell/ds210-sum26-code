// Run with: cargo run --bin inactive_reference
//
// This compiles and runs: we print ref0 before mutating the
// vector. Rust realizes ref0 is no longer used after printing,
// so it is no longer active, and we can mutate v by pushing.
fn main() {
    let mut v: Vec<String> = vec![String::from("str1"), String::from("str2")];
    // reference to the first element.
    let ref0: &String = &v[0];
    println!("{}", ref0);

    for i in 0..10 {
        v.push(format!("str{}", i));
    }

    println!("done");
}
