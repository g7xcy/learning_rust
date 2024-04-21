fn main() {
    let mut v: Vec<i32> = Vec::new();

    for i in 0..10 {
        v.push(i);
    }

    for i in 0..10 {
        println!("{}", &v[i]);
    }

    for i in 0..20 {
        let x: Option<&i32> = v.get(i);
        match x {
            Some(num) => println!("{num}"),
            None => println!("None")
        }
    }

    for i in &mut v{
        *i += 10;
    }

    for i in &v {
        println!("{}", i);
    }
}
