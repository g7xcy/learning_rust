// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist.");
        }

        pub fn seat_at_table() {
            println!("Please seat at the table.");
        }
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {
    println!("Deliver order!");
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    println!("父模块中的项不能访问子模块中的私有项；子模块中的项可以访问父模块中的项。");
    println!("模块的公有并不会默认令其内容公有。");
    front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::seat_at_table();    
}
