pub fn _slice_type() {
    let mut s = String::from("Hello world");

    let word = _first_word(&s);

    println!("{}", word);

    s.clear();

    s = String::from("Hello world");

    let hello = &s[..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn _first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
