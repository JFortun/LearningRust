pub fn _variables() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 2;
    println!("The value of y is: {}", y);
    let y = y * 2;
    println!("The value of y is: {}", y);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("There are {} spaces", spaces);

    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}
