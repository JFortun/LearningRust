pub fn _functions() {
    _another_function();
    _tell_number(15, 20);
    _expression();
    let x = _arrow_return();
    println!("(arrow) The value of x is {}", x);
    let x = _normal_return();
    println!("(return) The value of x is {}", x);
}

fn _another_function() {
    println!("Hello, functions!");
}

fn _tell_number(x: i32, y: i32) {
    println!("The number x is {}", x);
    println!("The number y is {}", y);
}

fn _statement() {
    let _x = 6;
}

fn _expression() {
    let _a = 5;

    let b = {
        let a = 3;
        a + 1
    };

    println!("The number b is {}", b);
}

fn _arrow_return() -> i32 {
    5 + 5
}

fn _normal_return() -> i32 {
    return 5;
}
