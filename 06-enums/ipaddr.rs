#[derive(Debug)]
struct IPV4(u8, u8, u8, u8);

#[derive(Debug)]
struct IPV6(String);

#[derive(Debug)]
enum IPAddr {
    V4(IPV4),  // enums can carry a variety of data, even other enums!
    V6(IPV6),
}

fn main() {
    let localhost: IPAddr = IPAddr::V4(IPV4(127, 0, 0, 1)); 
    let loopback: IPAddr = IPAddr::V6(IPV6(String::from("::1")));

    println!("{:?}, {:?}", localhost, loopback);
}
