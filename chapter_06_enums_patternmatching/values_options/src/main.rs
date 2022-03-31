// fn main() {
//     let localhost: IpAddrKind::V4(127, 0, 0, 1);

// }

// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// enum Message {
//     Quit,
//     Move { x:i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn some_function() {
//         println!("Lets get rust!")
//     }
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }


// ###################

// fn main() {
//     enum Option<T> {
//         Some(T),
//         None,
//     }
// }


// ###############

// fn main() {
//     value_in_cents(Coin::Quarter(UsState::Alaska));
// }

// #[derive(Debug)] // so we can inspect the state when compiling
// enum UsState {
//     Alabama,
//     Alaska,
//     Arizona,
//     Arkansas,
//     California,
//     //...
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => {
//             println!("State quarter from {:?}!", state);
//             25,
//         }
//     }
// }

// ########################

// fn main() {
//     let five: Option<i32> = Some(5);
//     let _six: Option<i32> = plus_one(five);
//     let _none: Option<i32> = plus_one(None);

// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i: i32) => Some(i + 1),
//     }
// }

// pub mod dice_roll;

// fn main() {
//     let some_value: Option<i32> = Some(3);
    
//     match some_value {
//         Some(3) => println!("three"),
//         _ => (),
//     }
    
//     if let Some(3) = some_value {
//         println!("three");
//     }
// }

// ################
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
