pub fn data_types() {
    let _guess: u32 = "42".parse().expect("Not a number!");

    let _x = 2.0;
    let _y: f32 = 3.0;

    let addition = 5 + 10;
    let subtraction = 95.5 - 10.3;
    let multiplication = 25 * 5;
    let division = 30.4 / 15.2;
    let remainder = 33 % 7;
    println!(
        "Addition: {}. Subtraction: {}. Multiplication: {}. Division: {}. Remainder: {}.",
        addition, subtraction, multiplication, division, remainder
    );

    let _t = true;
    let _f: bool = false;

    let _c = 'z';

    let tuple: (i32, f64, u8) = (500, 5.5, 1);
    let (x, y, z) = tuple;
    println!("{}, {}, {}", x, y, z);
    println!("{}, {}, {}", tuple.0, tuple.1, tuple.2);

    let _array = [1, 2, 3, 4, 5];
    let array2 = [5; 3];
    println!("{}{}{}", array2[0], array2[1], array2[2]);
    let _months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
}
