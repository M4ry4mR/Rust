// enum IpAddrKind {
//     V4(String),

//     V6(String),
// }
// // struct IpAddr {
// //     kind: IpAddrKind,
// //     address: String,
// // }
// fn route(ip_kind: IpAddrKind) {}
// fn main() {
//     // let four = IpAddrKind::V4;
//     // let six = IpAddrKind::V6;

//     // let home = IpAddr {
//     //     kind: IpAddrKind::V4,
//     //     address: String::from("127.0.0.1"),
//     // };

//     // let loopback = IpAddr {
//     //     kind: IpAddrKind::V6,
//     //     address: String::from("::1"),
//     // };
//     let home = IpAddr::V4(String::from("127.0.0.1"));

//     let loopback = IpAddr::V6(String::from("::1"));
// }

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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny =>{
            println!("good luck");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state)=>{
            println!("State quarter {:?}" , state);
            25
        }
    }
}
 fn main(){
    let x= value_in_cents(Coin :: Quarter);
    println!("The coin is equal to {} ", x)
 }