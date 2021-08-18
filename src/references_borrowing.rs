pub fn _references_borrowing() {
    let s1 = String::from("Hello");
    let len = _calculate_length(&s1);
    println!("The length of {} is {}", s1, len);

    let mut s = String::from("Hello");
    _change(&mut s);
    println!("{}", s);

    let reference = _no_dangle();
    println!("{}", reference);
}

fn _calculate_length(s: &String) -> usize {
    s.len()
}

fn _change(some_string: &mut String) {
    some_string.push_str(" world!");
}

fn _no_dangle() -> String {
    let s = String::from("Hello");

    s
}
