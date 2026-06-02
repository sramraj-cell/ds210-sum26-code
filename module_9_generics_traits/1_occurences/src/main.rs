pub fn count_occurences<T: PartialEq>(v: &Vec<T>, e: &T) -> usize {
    let mut count = 0;
    for element in v {
        if element == e {
            count += 1;
        }
    }
    return count;
}

#[derive(PartialEq)]
pub struct MyType {
    pub value: usize,
}

fn main() {
    let v1: Vec<i32> = vec![3, 1, 2, 3, 2, 2];
    let v2: Vec<String> = vec![String::from("hello"), String::from("world"), String::from("world!")];
    let v3 = vec![MyType { value: 10 }, MyType { value: 20 }, MyType { value: 10 }];

    println!("count of 2 in v1 {}", count_occurences(&v1, &2));
    println!("count of world in v2 {}", count_occurences(&v2, &String::from("world")));
    println!("count of value: 10 in v3 {}", count_occurences(&v3, &MyType { value: 10 }));
}
