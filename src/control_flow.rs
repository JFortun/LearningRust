#[derive(Debug)]
enum _State {
    _Alabama,
    _Alaska,
}

enum _Coin {
    _Penny,
    _Nickel,
    _Dime,
    _Quarter(_State),
}

pub fn _control_flow() {
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

    _value_in_cents(_Coin::_Penny);
    _value_in_cents(_Coin::_Quarter(_State::_Alabama));

    let _five = Some(5);
    let _six = _plus_one(_five);
    let _none = _plus_one(None);

    let _some_u8_value = 0u8;
    match _some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn _value_in_cents(coin: _Coin) -> u8 {
    match coin {
        _Coin::_Penny => {
            println!("Lucky penny!");
            1
        }
        _Coin::_Nickel => 5,
        _Coin::_Dime => 10,
        _Coin::_Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn _plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
