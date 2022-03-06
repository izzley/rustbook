struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1: User = User {
        email: String::from("yowhat@gmail.com"),
        username: String::from("yowie"),
        active: true,
        sign_in_count: 1

    };

    let name: String = user1.username;
    user1.username = String::from("wallace123");

    let user2: User = build_user(
        String::from("someone@gmail.com"),
        String::from("Biggs")
    );

    let user3: User = User {
        email: String::from("user3@gmail.com"),
        username: String::from("user3"),
        ..user2
    };
}


// field init shorthand syntax: args are same as struct field names
fn build_user(email: String, username: String) -> User {
    User {
        email, // shorthand for email: email,
        username,// username: username,
        active: true,
        sign_in_count: 1,
    }
}


// fn main() {
//     struct Color(i32, i32, i32);
//     struct Point(i32, i32, i32);
// }

// ####################


