
#[derive(Debug)]
enum IpAddrKind {
    V4,
    _V6,
}

#[derive(Debug)]
struct IpAddr {
    addr: String,
    kind: IpAddrKind,
}

#[derive(Debug)]
enum IpAddr2 {
    V4_2(u8, u8, u8, u8),
  //  V6_2(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {
                println!("Quit");
            }

            Message::Move { x, y } => {
                println!("Move: {} {}", x, y);
            }

            Message::Write (s) => {
                println!("Write: {}", s);
            }

            Message::ChangeColor (f, s, t) => {
                println!("ChangeColor: {} {} {}", f, s, t);
            }
        }
    }
}

fn main() {

    let tc_lx = IpAddr {
      addr: String::from("192.168.11.46"),
      kind: IpAddrKind::V4,
    };

    println!("{:#?}", tc_lx);

    let tc_lx = IpAddr2::V4_2(10, 0, 11, 16);

    println!("{:#?}", tc_lx);

    match tc_lx {
        IpAddr2::V4_2 (a, _b, _c, _d) => {
            println!("first u: {}", a);
        }
    }

    let m = Message::Quit;
    m.call();
    let m = Message::Move { x: 1, y: 2 };
    m.call();
    let m = Message::ChangeColor (1, 2, 3);
    m.call();
    let m = Message::Write ( String::from("blue"));
    m.call();
}
