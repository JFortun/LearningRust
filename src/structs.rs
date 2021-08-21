pub fn structs() {
    let mut user1 = User {
        _username: String::from("Kevin"),
        _email: String::from("kevin@mail.com"),
        _sign_in_count: 1,
        _active: true,
    };

    println!("EMail: {}", user1._email);
    user1._email = String::from("kevin2@mail.com");
    println!("EMail: {}", user1._email);

    let user2 = _build_user(String::from("John"), String::from("john@mail.com"));
    println!("Name: {} | EMail: {}", user2._username, user2._email);

    let user3 = User {
        _username: String::from("Clare"),
        _email: String::from("clare@mail.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

struct User {
    _username: String,
    _email: String,
    _sign_in_count: u64,
    _active: bool,
}

fn _build_user(_username: String, _email: String) -> User {
    User {
        _username,
        _email,
        _sign_in_count: 1,
        _active: true,
    }
}
