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
    fn print_kind(&self) {
        match self {
            IpAddrKind::V4 => {
                println!("v4!")
            }
            IpAddrKind::V6 => {
                println!("v6!")
            }
        }
    }

    fn echo(s: &str) {
        println!("{}", s);
    }
}

#[derive(Debug)]
enum IpAddrKindv2 {
    V4(String),
    V6(String),
}

impl IpAddrKindv2 {
    fn print_kind(&self) {
        match self {
            IpAddrKindv2::V4(_) => {
                println!("v4!")
            }
            IpAddrKindv2::V6(_) => {
                println!("v6!")
            }
        }
    }
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
    IpAddrKind::echo("I'm an echo!");
    loopback.kind.print_kind();

    let optional_0: Option<usize> = None;
    let optional_1 = Some(2);
    handle_optional_size(optional_0);
    handle_optional_size(optional_1);
    home_v2.print_kind();
}

fn handle_optional_size(input: Option<usize>) {
    match input {
        Some(num) => {
            println!("Something!: {}", num)
        }
        None => {
            println!("None!");
        }
    }
}
