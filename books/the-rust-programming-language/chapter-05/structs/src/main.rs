fn main() {
    let u1 = User {
        username: String::from("User 1"),
        email: String::from("user@example.com"),
        active: true,
        sign_in_count: 1
    };

    let u2 = User {
        username: String::from("User 2"),
        ..u1
    };

    // This produces E0382 because the email was moved:
    // println!("{}", u1.email);

    // Although the values are the same, the instance come from different tuple
    // structs, therefore they're different. A function that requires a Color
    // instance won't accept Point (and vice versa).
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);