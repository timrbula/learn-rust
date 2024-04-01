#[derive(Debug)]
enum Language {
    TYPESCRIPT,
    PYTHON,
    GO,
    CPP,
    C,
    RUST,
    JAVASCRIPT,
    JAVA,
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

impl IpAddrKind {
    fn echo(s: &str) {
        println!("{}", s);
    }
}

#[derive(Debug)]
enum IpAddrKindv2 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

struct LangParser {
    language: Language,
    enabled: bool,
}

fn main() {
    let js = Language::JAVASCRIPT;
    let rust = Language::RUST;
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home_v2 = IpAddrKindv2::V4(String::from("127.0.0.1"));

    println!("{:?}", js);
    println!("Hello, world!");
    println!("{:?}", home);
    println!("{:?}", loopback);
    println!("{:?}", home_v2);
    IpAddrKind::echo("what!");
}
