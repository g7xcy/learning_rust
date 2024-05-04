use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 11);
    scores.insert(String::from("Blue"), 15);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);
    // println!("Score of team {} is {}.", team_name, score);
    scores.entry(String::from("Yellow")).or_insert(20);

    println!("{:?}", scores);

    scores.insert(String::from("Red"), 100);
    let yellow_score = scores.entry(String::from("Yellow")).or_insert(0);
    // *yellow_score = 1234;
    println!("{yellow_score}");
    println!("{:?}", scores);

    let blue_score = scores.enrty(String::From("Blue")).or_insert(0);
    println!("{blue_score}");
}
