/* 枚举：
    定义枚举：
    IP 地址：
        enum IpAddrKind {
            V4,
            V6,
        }

    枚举值：
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
 */
enum IpAddrKind {
    V4,
    V6,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopack = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let q = Message::Quit;
    let m = Message::Move {x:12, y:24};
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(0, 0, 0);
    m.call();
}

fn route(ip_kind: IpAddrKind) {}

/*把数据附加到枚举类型中
    enum IpAddr {
        V4(String),
        V6(String),
    }

    优点：
        - 不需要额外使用 struct
        - 每个变体可以拥有不同的类型以及关联的数据量

    例如：

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
 */

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 为枚举定义方法
impl Message {
    fn call(&self) {}
}