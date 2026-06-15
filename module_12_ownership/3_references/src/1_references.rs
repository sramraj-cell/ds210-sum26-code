// Run with: cargo run --bin references
//
// References are really similar to pointers: they are also
// based on addresses. But unlike pointers, they are safe to use!
fn main() {
    let x1: i32 = 10;
    let ref_x1: &i32 = &x1;
    println!("address of x1 {:p}", &x1);
    println!("ref_x1 refers to address {:p}", ref_x1);
    println!("ref_x1 refers to value {}", ref_x1);
}
