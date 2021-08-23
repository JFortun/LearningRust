struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

struct User {
    _username: String,
    _email: String,
    _sign_in_count: u64,
    _active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

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

    let _user3 = User {
        _username: String::from("Clare"),
        _email: String::from("clare@mail.com"),
        ..user1
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1: {:#?}", rect1);

    println!("The area of the rectangle is {}", area(&rect1));
}

fn _build_user(_username: String, _email: String) -> User {
    User {
        _username,
        _email,
        _sign_in_count: 1,
        _active: true,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
