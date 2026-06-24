fn longest<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() >= y.len() {
        return x;
    } else {
        return y;
    }
}
fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let longest = longest(&string1, &string2);

    //drop(string1);
    //drop(string2);

    println!("{longest}");
}
