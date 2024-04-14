struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user = User {
        active: false,
        username: String::from("frederica"),
        email: String::from("frederica@test.com"),
        sign_in_count: 1,
    };

    println!("{}", user.email);

    user.email = String::from("new@test.com");
    println!("{}", user.email);

    let new_user = build_user(String::from("Alex"), String::from("alex@test.com"));
    println!("{}", new_user.email);

    let new_user2 = User {
        username: String::from("new user2"),
        email: String::from("new_user2@test.com"),
        ..new_user
    };
    println!("{}", new_user2.email);
    println!(
        "Cannot use new_user. Because `new_user.username` was moved to `new_user2`, not copy to."
    );
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
