fn main() {
    println!("create是rust编译时的最小代码单位.");
    println!("二进制create包含main函数；库create没有。");
    println!("package中至多一个库，任意多个binary，至少一个create。");
    println!("src/main.rs是binary create root，src/lib.rs是library create root。");
}
