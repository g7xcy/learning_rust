fn main() {
    println!("Null is a value which stands for `no value`.\nOption is a very good type.\n");

    println!("I call it my billion-dollar mistake. At that time, I was designing the first comprehensive type system for references in an object-oriented language. My goal was to ensure that all use of references should be absolutely safe, with checking performed automatically by the compiler. But I couldn't resist the temptation to put in a null reference, simply because it was so easy to implement. This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.\n--- Tony Hoare");

    // let some_integer = Some(5);
    // let some_string = Some(String::from("Frederica"));
    // let some_bool = Some(true);
    // let absent_iunteger: Option<i32> = None;

    // let absent_byte: Option<char> = None;

    let x: i32 = 17;
    let y: Option<i32> = Some(25);
    match y {
        Some(num) => println!("{x} + {num} = {}", x + num),
        None => println!("y is None!"),
    }
    let y: Option<i32> = None;
    match y {
        Some(num) => println!("{x} + {num} = {}", x + num),
        None => println!("y is None!"),
    }
}
