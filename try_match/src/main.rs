enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let coin: Coin = Coin::Penny;
    println!("Value of coin is {}", value_in_cents(&coin));
    let coin: Coin = Coin::Nickel;
    println!("Value of coin is {}", value_in_cents(&coin));
    let coin: Coin = Coin::Dime;
    println!("Value of coin is {}", value_in_cents(&coin));
    let coin: Coin = Coin::Quarter;
    println!("Value of coin is {}", value_in_cents(&coin));
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
