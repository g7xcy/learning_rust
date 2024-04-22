fn main() {
    println!("rust只有一种字符串类型：slice str -> &str");

    let s = "hello world".to_string();
    println!("{s}");

    let mut s = String::from("你好！");
    println!("{s}");
    s.push_str(", rust!");
    println!("{s}");

    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s1 = s1 + &s2;
    println!("String::+会获取调用者的所有权！");
    println!("{s1}");

    let s1 = String::from("do");
    let s2 = String::from("re");
    let s3 = String::from("mi");
    let s = format!("{s1}-{s2}-{s3}");
    println!("format! macro just likes println! macro; format! macro use reference");
    println!("{s}");

    let s = String::from("Rust的字符串不支持索引。String本质是Vec<u8>，UTF-8每个字符占用的字节数不固定。同时性能也无法保证。");
    println!("{s}");

    for c in s.chars() {
        println!("{c}");
    }
    for b in s.bytes() {
        println!("{b}");
    }
}
