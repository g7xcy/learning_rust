fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{s}");
}

fn change(ref_s: &mut String) {
    ref_s.push_str(" world!");
}
