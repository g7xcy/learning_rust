struct vec3i(i32, i32, i32);
struct vec3f(f32, f32, f32);

fn main() {
    let v1 = vec3i(0, 1, 2);
    let v2 = vec3f(1.0, 1.1, 1.2);
    println!("({}, {}, {})", v1.0, v1.1, v1.2);
    println!("({}, {}, {})", v2.0, v2.1, v2.2);
}
