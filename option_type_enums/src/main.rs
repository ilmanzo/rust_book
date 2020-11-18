enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Colorado,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>  {
            println!("A quarter from {:?}",state);
            25} , 
    }
}

fn main() {
    let c=Coin::Quarter(UsState::Colorado);
    let x=value_in_cents(c);
    println!("value in cents = {}",x);
}
