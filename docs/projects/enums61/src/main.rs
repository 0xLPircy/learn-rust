// enum IpAddrKind {
//     V4,
//     V6,
// }
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
    // Move { x: i32, y: i32 },
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// struct IpAddr {
//     kind: IpAddrKind, //can use in a struct
//     address: String,
// }
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}


fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
        let m = Message::Write(String::from("hello"));
        m.call();
}
// fn route(ip_kind: IpAddrKind) {}