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

    let addrV4 = IpAddr2::V4(127, 5, 6, 7);
    let addrV6 = IpAddr2::V6(String::from("Some ipv6 data"));
    dbg!(&addrV4);
    dbg!(&addrV6);


}
