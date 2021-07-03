#[derive(Debug)]
enum UsState {
    Alabama,
    Arkansas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Quarter(UsState::Alabama);
    println!("The value of the Coin is: {}", value_in_cents(coin));

    let x = Some(5);
    let six = plus_one(x);
    let none = plus_one(None);

    println!("Six is: {:?}", six);
    println!("None is: {:?}", none);

    let some_u8_value = Some(8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("The state is: {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
