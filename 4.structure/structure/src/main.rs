#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    sign_in_count: u64,
}

fn build_user(username: String) -> User {
    User {
        username,  // => username = username
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // 整个实例都是可变的
    let user1 = build_user(String::from("someuser"));

    println!("{:?}", user1);
}
