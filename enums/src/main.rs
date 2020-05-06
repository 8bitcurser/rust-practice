
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}



fn main() {
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
    println!("{}", value_in_cents(Coin::Dime));
}
