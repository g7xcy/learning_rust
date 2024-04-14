#[derive(Debug)]
struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let p = Point3 { x: 3, y: 4, z: 5 };
    println!("{:#?}", p);
}
