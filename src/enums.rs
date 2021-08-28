enum _IpAddress {
    _V4(u8, u8, u8, u8),
    _V6(String),
}

enum _Message {
    _Quit,
    _Move { x: i32, y: i32 },
    _Write(String),
    _ChangeColor(i32, i32, i32),
}

impl _Message {
    fn _call(&self) {
        // method body would be defined here
    }
}

pub fn _enums() {
    let _home = _IpAddress::_V4(127, 0, 0, 1);
    let _loopback = _IpAddress::_V6(String::from("::1"));

    let m = _Message::_Write(String::from("hello"));
    m._call();

    let _some_number = Some(5);
    let _some_string = Some("a string");

    let _absent_number: Option<i32> = None;
}
