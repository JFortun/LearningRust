pub fn _ownership() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{}, world!", s2);

    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let string = String::from("Hello");
    _takes_ownership(string);

    let x = 5;
    _makes_copy(x);

    let _string1 = _gives_ownership();
    let string2 = String::from("Hello");
    let _string3 = _takes_and_gives_back(string2);

    let string_1 = String::from("Hello");
    let (string_2, len) = _calculate_length(string_1);
    println!("The length of {} is {}", string_2, len);
}

fn _takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn _makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn _gives_ownership() -> String {
    let some_string = String::from("Hello");

    some_string
}

fn _takes_and_gives_back(string: String) -> String {
    string
}

fn _calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
