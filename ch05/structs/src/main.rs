struct Color(i32, i32, i32);

struct User {
    username: String,
    email: String,
    eye_color: Color,
}

fn main() {
    let mut avi: User = build_user(
        String::from("avi@gmail.com"),
        String::from("avi"),
        Color(0, 0, 255),
    );
    println!("{}", avi.email);
}

fn build_user(email: String, username: String, eye_color: Color) -> User {
    User {
        email,
        username,
        eye_color,
    }
}
