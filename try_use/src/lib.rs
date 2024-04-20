mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    println!("use只在当前作用域内生效。");
    hosting::add_to_waitlist();
    println!("use使用到函数的父模块。这样既可以使路径最小，也可以清楚的知道函数并不是当前模块定义的。");
    println!("在use引入结构体、枚举等数据、结构时，引用到完整路径是个方便的选择。");
    println!("注意，rust不允许将两个名称相同的项带入作用域。哪怕这两个项的路径不同。这种情况下可以使用as重命名。");
    println!("使用pub use可以实现重导出：别人可以导入use导入的项，仿佛其被定义在当前作用域。");
}

mod customer {
    pub fn new_rat_at_restaurant() {
        // hosting::add_to_waitlist();
        // crate::front_of_house::hosting::add_to_waitlist();
        super::front_of_house::hosting::add_to_waitlist();
    }
}
