use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    let words = "Hello wonderful world, Hello world!";

    println!("or_insert方法返回值的可变引用！")

    for word in words.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
