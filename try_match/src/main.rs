enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn main() {
    println!("match模式匹配后面的表达式不要求结果为bool类型。");
    println!("match匹配分支的表达式结果是match的返回值。");
    println!(
        "match分支如果使用了大括号`{}`,则分支末尾的逗号`,`可以省略。\n",
        "{}"
    );
    let coin: Coin = Coin::Penny;
    println!("Value of coin is {}", coin.value_in_cents());

    let coin: Coin = Coin::Nickel;
    println!("Value of coin is {}", coin.value_in_cents());

    let coin: Coin = Coin::Dime;
    println!("Value of coin is {}", coin.value_in_cents());

    let coin: Coin = Coin::Quarter;
    println!("Value of coin is {}", coin.value_in_cents());
}
