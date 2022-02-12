struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("svpra.prog@gmail.com"),
        username: String::from("svpra"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("svpra.prog@gmail.com");
}
