// // enum IpAddrKind {
// //         V4,
// //         V6,
// //     }

// // struct IpAddr {
// //         kind: IpAddrKind,
// //         address: String,
// //     }

// // let home = IpAddr {
// //         kind: IpAddrKind::V4,
// //         address: String::from("127.0.0.1"),
// //     };

// // let loopback = IpAddr {
// //         kind: IpAddrKind::V6,
// //         address: String::from("::1"),
// //     };


// // Equiv to:

// enum IpAddr {
//         V4(u8, u8, u8, u8), // four numeric components that will have values between 0 and 255
//         V6(String),
//     }

// let home = IpAddr::V4(String::from("127.0.0.1"));

// let loopback = IpAddr::V6(String::from("::1"));

struct Ipv4Addr { //standard lib has stuff to store IP addresses
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// -------------

// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

// Equiv to:

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message { //implementation for type message
        fn call(&self) {
            // method body would be defined here
        }
    }

let m = Message::Write(String::from("hello"));
m.call();
