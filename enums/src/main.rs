// #[derive(Debug)]

// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAdress {
//     address: String,
//     kind: IpAddrKind,
// }

// fn main() {

//     let google_address = IpAdress {
//         address: String::from("1.2.3.4"),
//         kind: IpAddrKind::V4,
//     };
//     route(google_address);
// }

// fn route(ip: IpAdress) {
//     println!("Routing request to IP {} of kind {:?}", ip.address, ip.kind);

// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     // if we insert boolean it will become bool
//     // let op = Some(true);

//     // if we insert string it will become string
//     let op = Some(String::from("abc"));

//     // let op: Option<i8> = Some(5);


// }

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    println!("Value is {}", match_in_cent(coin));
}

fn match_in_cent(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState::Alaska) => {
            println!("Hello from Alaska");
            2
        }
        Coin::Quarter(state) =>{
            println!("Got Q of value {:?}", state);
            25
        }
    }
}
