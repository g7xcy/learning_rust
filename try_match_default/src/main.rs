enum WeekDays {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

fn shall_i_work_today(day: &WeekDays) {
    match day {
        WeekDays::Sunday => println!("Happy weekend!"),
        WeekDays::Saturday => println!("Happy weekend!"),
        _ => println!("Oh no! I must go to work!"),
    }
}

fn main() {
    let today = WeekDays::Monday;
    shall_i_work_today(&today);

    let today = WeekDays::Sunday;
    shall_i_work_today(&today);
}
