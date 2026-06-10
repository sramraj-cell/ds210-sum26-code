// Run with: cargo run --bin dangling_pointer
//
// This code crashes at runtime: the pointer becomes dangling
// after the vector resizes.
fn main() {
    let mut v: Vec<String> = vec![String::from("str1"), String::from("str2")];
    // pointer to the first element.
    let ptr0: *const String = &v[0] as *const String;

    // We are inserting many elements to the vector.
    // This causes the vector to resize and changes the location
    // of its elements in memory.
    for i in 0..10 {
        v.push(format!("str{}", i));
    }

    // Now, the old ptr address is no longer valid.
    println!("address of first element used to be {:p}", ptr0);
    println!("address of first element became {:p}", &v[0]);

    unsafe {
        println!("dereferencing the pointer");
        println!("{}", *ptr0);
        println!("program done!");
    }
}
