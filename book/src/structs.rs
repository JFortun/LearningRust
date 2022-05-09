struct _Color(i32, i32, i32);

struct _Point(i32, i32, i32);

struct _User {
    _username: String,
    _email: String,
    _sign_in_count: u64,
    _active: bool,
}

#[derive(Debug)]
struct _Rectangle {
    _width: u32,
    _height: u32,
}

impl _Rectangle {
    fn _area(&self) -> u32 {
        self._width * self._height
    }

    fn _can_hold(&self, other: &_Rectangle) -> bool {
        self._width > other._width && self._height > other._height
    }

    fn _square(size: u32) -> _Rectangle {
        _Rectangle {
            _width: size,
            _height: size,
        }
    }
}

pub fn _structs() {
    let mut user1 = _User {
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

    let _user3 = _User {
        _username: String::from("Clare"),
        _email: String::from("clare@mail.com"),
        ..user1
    };

    let _black = _Color(0, 0, 0);
    let _origin = _Point(0, 0, 0);

    let rect1 = _Rectangle {
        _width: 30,
        _height: 50,
    };
    let rect2 = _Rectangle {
        _width: 10,
        _height: 40,
    };
    let rect3 = _Rectangle {
        _width: 60,
        _height: 45,
    };
    println!("rect1: {:#?}", rect1);

    println!("The area of the rectangle is {}", rect1._area());

    println!("Can rect1 hold rect2? {}", rect1._can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1._can_hold(&rect3));

    let square = _Rectangle::_square(5);
    println!("square: {:#?}", square);
}

fn _build_user(_username: String, _email: String) -> _User {
    _User {
        _username,
        _email,
        _sign_in_count: 1,
        _active: true,
    }
}
