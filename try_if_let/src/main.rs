use std::fmt::Display;

#[derive(Debug)]
enum Dice {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

enum Weekday {
    Monday,
    Tuesday,
    Saturday,
}

fn main() {
    let dice = Some(Dice::Three);

    println!("if let的关键是将匹配的结果和模式反过来写。只希望匹配到某一结果，而在其他结果时无动作，则可以时使用if let。");
    println!("但是，代价是什么呢？代价是会失去match的穷尽性检查！");
    if let Some(cur_num) = dice {
        println!("{:?}", cur_num);
    }

    let today = Weekday::Monday;
    if let Weekday::Monday = today {
        println!("It is Monday today.");
    }

    let today = Weekday::Tuesday;
    if let Weekday::Tuesday = today {
        println!("It is Tuesday today.");
    }

    let today = Weekday::Saturday;
    if let Weekday::Saturday = today {
        println!("It is Saturday today.");
    }

    let coin = 5;
    if let 5 = coin {
        println!("The value of coin is 5");
    } else {
        println!("else branch of `if let`");
    }

    let coin = 18;
    if let 5 = coin {
        println!("The value of coin is 5");
    } else {
        println!("else branch of `if let`");
    }
}
