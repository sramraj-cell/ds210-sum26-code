use std::collections::HashMap;

fn main() {
    // Let us turn the names and grades into a hashmap!
    let map = HashMap::from([
        ("Kinan", 0),
        ("Matt", 100),
        ("Taishan", 95),
        ("Zach", 88),
        ("Kesar", 99),
        ("Lingie", 98),
        ("Emir", 97)
    ]);

    // Now, map[<name>] gives us the grade without having to search for the name!
    let target = "tom";
    let grade = map.get(target);
    match grade {
        Some(value) => println!("{value}"),
        None=> println!("not found")
    }
    
    let target = "Kinan";
    let grade = map.get(target);
    match grade {
        Some(value) => println!("{value}"),
        None=> println!("not found")
    }
    
    let target = "Kesar";
    let grade = map.get(target);
    match grade {
        Some(value) => println!("{value}"),
        None=> println!("not found")
    }
}
