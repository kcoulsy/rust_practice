enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String, String, String, String),
}

struct IpAddr {
    kind: IpAddrKind,
}

// enums can hold different types of data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn print(&self) {
        println!("message is {self:?}");
    }
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6("::1".to_string(), "::1".to_string(), "::1".to_string(), "::1".to_string());

    println!("four is {four:?}, six is {six:?}");

    let message = Message::Write(String::from("hello"));
    message.print();

    // optional enums
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
    let absent_char: Option<char> = None;

    println!("some_number is {some_number:?}, some_char is {some_char:?}, absent_number is {absent_number:?}, absent_char is {absent_char:?}");
}


fn route(ip_kind: IpAddrKind) {
    println!("routing to {ip_kind:?}");
}