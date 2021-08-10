pub fn functions() {
    another_function();
    tell_number(15, 20);
    expression();
    let x = arrow_return();
    println!("(arrow) The value of x is {}", x);
    let x = normal_return();
    println!("(return) The value of x is {}", x);
}

fn another_function() {
    println!("Hello, functions!");
}

fn tell_number(x: i32, y: i32) {
    println!("The number x is {}", x);
    println!("The number y is {}", y);
}

fn _statement() {
    let _x = 6;
}

fn expression() {
    let _a = 5;

    let b = {
        let a = 3;
        a + 1
    };

    println!("The number b is {}", b);
}

fn arrow_return() -> i32 {
    5 + 5
}

fn normal_return() -> i32 {
    return 5;
}
