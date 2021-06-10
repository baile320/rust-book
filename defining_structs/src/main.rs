fn main() {
    println!("Hello, world!");

    let mut tyler = User {
        username: String::from("tyler"),
        email: String::from("tyle@tyler.com"),
        sign_in_count: 0,
        active: true,
    };

    tyler.email = String::from("tyler@tyler.com");

    let user1 = build_user(
        String::from("user1"),
        String::from("user1isntreal@user1isntreal.net"),
    );

    let user2 = User {
        username: String::from("tyler2"),
        email: String::from("tyler2@tyler2.com"),
        ..tyler
    };
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 0,
        active: true,
    }
}
