mod back_of_house {
    pub struct Lunch {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Lunch {
        pub fn autumn(toast: &str) -> Lunch {
            Lunch {
                toast: String::from(toast),
                seasonal_fruit: String::from("grape")
            }
        }
    }
}

pub fn eat_at_restaurant(toast: &str) {
    let mut meal = back_of_house::Lunch::autumn(toast);
    println!("I'd like {} toast please.", meal.toast);
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);

    // meal.seasonal_fruit = String::from("test");
}
