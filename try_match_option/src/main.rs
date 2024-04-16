fn plus_one(x: &Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(num) => Some(num + 1),
    }
}

fn print_option_i32(x: &Option<i32>) {
    match x {
        None => println!("None"),
        Some(num) => println!("{num}"),
    }
}

fn main() {
    println!("match必须覆盖Option的所有情况：Some和None。");

    let three = Some(3);
    let four = plus_one(&three);
    let none = plus_one(&None);

    print_option_i32(&three);
    print_option_i32(&four);
    print_option_i32(&none);
}
