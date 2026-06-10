// Run with: cargo run --bin missing_write
//
// This does not compile (on purpose): x has R and O permissions
// but no W (because it is not mut). The line x = x + 1 requires
// both R and W. The fix is to change the code to `let mut x`,
// because that change adds W permissions to x!
fn main() {
    let x: i32 = 10;
    x = x + 1;
    println!("{}", x);
}
