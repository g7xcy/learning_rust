fn main() {
    let s = String::from("hello world");
    let s2 = String::from("hello");
    println!("{}, {}", first_world(&s), first_world(&s2));

    let s = String::from("Hello World!");
    println!("{}", s.len());
    println!("{}", &s.len());
    println!("{}", s[..].len());

    println!("{}", s.capacity());
    println!("{}", &s.capacity());
    //   println!("{}", s[..].capacity());
    println!("Reference has capacity method, slice not.");
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes(); // transform string to an array of byte

    for (i, &item) in bytes.iter().enumerate() {
        // iter(): create an iterator on the array
        if item == b' ' {
            return &s[..i + 1];
        }
    }
    &s[..]
}
