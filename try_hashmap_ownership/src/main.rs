use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    let key = String::from("1");
    let value = 1;

    println!("HashMap will take the ownership of key and value.");

    map.insert(&key, value);

    println!("{key}");

    for (k, v) in map {
        println!("{k}: {v}");
    }
}
