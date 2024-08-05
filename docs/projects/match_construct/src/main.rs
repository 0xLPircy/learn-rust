#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin, x: u32, y:Option<i32>) -> u8 {
    // match coin {
    //     Coin::Penny => {
    //         println!("Lucky penny!");
    //         1
    //     },
    //     Coin::Nickel => 5,
    //     Coin::Dime => 10,
    //     Coin::Quarter(UsState::Alaska) => {
    //         // println!("State quarter from {state:?}!");
    //         25
    //     }
    //     Coin::Quarter(UsState::Alabama) => {
    //         // println!("State quarter from {state:?}!");
    //         26
    //     }
    // }
    match x {
        6 =>1,
        // 0_u32..=5_u32 | 7_u32..=u32::MAX => todo!()
        other => 0
    }
    // match y {
    //     None => None,
    //     Some(i) => Some(i + 1),
    // }
}

fn main() {
    let qwe=Coin::Quarter(UsState::Alaska);
    let x = 3;
    let y: Option<i32> = Some(5);
    println!("{}", value_in_cents(qwe, x, y));
}
