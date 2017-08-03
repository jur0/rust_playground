
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr1 {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Like structs, enums can have impl block.
impl IpAddr2 {
    fn new_ip4(a: u8, b: u8, c: u8, d: u8) -> IpAddr2 {
        IpAddr2::V4(a, b, c, d)
    }

    fn new_ip6(addr: String) -> IpAddr2 {
        IpAddr2::V6(addr)
    }
}


fn main() {
    {
        let home = IpAddr1 {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };
        let loopback = IpAddr1 {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };

        println!("home = {:?}", home);
        println!("loopback = {:?}", loopback);
    }

    {
        let home = IpAddr2::V4(127, 0, 0, 1);
        let loopback = IpAddr2::V6(String::from("::1"));

        println!("home = {:?}", home);
        println!("loopback = {:?}", loopback);
    }

    {
        let home = IpAddr2::new_ip4(10, 10, 10, 10);
        let loopback = IpAddr2::new_ip6(String::from("::1"));

        println!("home = {:?}", home);
        println!("loopback = {:?}", loopback);
    }
}
