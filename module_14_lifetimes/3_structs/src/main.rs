#[derive(Debug)]
struct ImportantElement<'a> {
    element: &'a String,
    index: usize,
}

fn main() {
    let strings = vec![
        String::from("hello"),
        String::from("goodbye")
    ];

    let e = ImportantElement {
        element: &strings[0],
        index: 0
    };

    //drop(strings);

    println!("{:?}", e);
}
