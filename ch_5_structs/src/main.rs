trait AddOne {
    fn increment_sign_in_count(&self);
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: usize,
}

impl User {
    fn increment_sign_in_count(&mut self) -> () {
        self.sign_in_count += 1;
        println!("{}", self.sign_in_count);
    }
}
#[derive(Debug)]
struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Rect {
    fn is_square(&self) -> bool {
        return self.width == self.height;
    }

    fn can_hold(&self, other: &Rect) -> bool {
        return self.width >= other.width && self.height >= other.height;
    }
}

fn run() {
    let rect1 = Rect {
        x: 1,
        y: 1,
        width: 12,
        height: 12,
    };

    let rect2 = Rect { width: 11, ..rect1 };

    println!("{:?}", rect1);

    let is_square = rect1.is_square();

    println!(
        "Is rect1 a square?: {}",
        if is_square { "yes" } else { "no" }
    );
    println!(
        "Can rect1 hold rect2?: {}",
        if rect1.can_hold(&rect2) { "yes" } else { "no" }
    );
}

fn get_rachel() -> User {
    let user = User {
        username: String::from("RachelRobz"),
        active: true,
        email: String::from("rachel@gmail.com"),
        sign_in_count: 11,
    };
    return user;
}

fn build_user(username: String, email: String) -> User {
    return User {
        username,
        email,
        sign_in_count: 0,
        active: true,
    };
}

fn main() {
    run();
    let mut rachel = get_rachel();
    let tim = User {
        username: String::from("timrbula"),
        ..get_rachel()
    };
    let mut friend = build_user(String::from("friend"), String::from("friend@mail.com"));
    rachel.email.push_str("a");
    println!("{}", rachel.username);
    println!("{:p}", &rachel.email);
    println!("{}", tim.username);
    println!("{:p}", &tim.email);
    println!("{}", friend.username);
    println!("{}", friend.email);
    friend.increment_sign_in_count();
}
