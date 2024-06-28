#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

#[derive(Debug)]
enum IpAddrKind2 {
    V4(String), // Here is V4(String) because V4 is actually a function that returns an instance of
                // IpAddrKind2
    V6(String),
}

#[derive(Debug)]
enum IpAddr2 {
    V4(u8, u8, u8, u8), // Enums do not care about the different data
    V6(String),
}

// WHAT WE DO HERE IS GO BACK BACK BACK BACK
#[derive(Debug)]
enum Message {
    Close,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn Call(&self) {
        match self {
            Message::Write(text) => println!("{}", text),
            Message::ChangeColor(r,b,g) => println!("{r} {b} {g}"),
            _ => (),
        }
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    dbg!(&four);
    dbg!(&six);

    // This is how you will define a group data with enum + struct
    let home = IpAddr {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };
    dbg!(&home);

    // This is instead by using just the enum
    let add = IpAddrKind2::V4(String::from("127.0.0.2"));
    dbg!(&add);

    let addr_v4 = IpAddr2::V4(127, 5, 6, 7);
    let addr_v6 = IpAddr2::V6(String::from("Some ipv6 data"));
    dbg!(&addr_v4);
    dbg!(&addr_v6);

    let msg = Message::Write(String::from("Something!"));
    let color = Message::ChangeColor(1,2,3);
    msg.Call();
    color.Call();

    let some_num = Some(2);
    let some_char = Some('c');
    let some_null: Option<i32> = None;
    dbg!(&some_num);
    dbg!(&some_char);
    dbg!(&some_null);

    let the_num = some_num.unwrap();
    let new_num = the_num + 2;
    dbg!(&new_num);
    
}
