use std::time::Instant;

mod helpers;

fn pass1(v: &Vec<i32>) {
    let i = helpers::random_index(v.len());
    println!("{}", v[i]);
}

fn pass2(v: Vec<i32>) {
    let i = helpers::random_index(v.len());
    println!("{}", v[i]);
}

fn main() {
    let v = helpers::random_vector(10_000_000);

    let time = Instant::now();
    pass1(&v);
    let duration1 = time.elapsed();

    let time = Instant::now();
    pass2(v.clone());
    let duration2 = time.elapsed();


    let time = Instant::now();
    pass2(v);
    let duration3 = time.elapsed();

    println!("pass1 {:?}", duration1);
    println!("pass2 {:?}", duration2);
    println!("pass3 {:?}", duration3);
    println!("done");
}
