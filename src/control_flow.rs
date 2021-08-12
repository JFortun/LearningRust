pub fn control_flow() {
    let number = 8;

    if number < 5 {
        println!("Condition was true");
    } else if number != 0 {
        println!("Number was something other than 0");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("Condition was false");
    }
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The number is {}", number);

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("End!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is {}", element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("End!");
}
