// Run with: cargo run --bin with_pointers
//
// This code crashes at runtime ("done" is never printed): the
// pointer e0 dangles after helper_function resizes the vector.
pub fn helper_function(v: &mut Vec<String>) {
    for i in 0..2 {
        v.push(format!("{}", i));
    }
}

pub fn main() {
    let mut v: Vec<String> = vec![String::from("hello"), String::from("bye")];
    let e0: *const String = &v[0] as *const String;

    helper_function(&mut v);

    println!("v = {:?}", v);
    println!("At first, address of the first element was {:p}", e0);
    println!("But now, the address of the first element is {:p}", &v[0]);
    unsafe {
        println!("e0 points to {}", *e0);
    }
    println!("done");
}
