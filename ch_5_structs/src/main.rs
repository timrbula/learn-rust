#[derive(Debug)]

struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32
}

impl Rect {
    fn is_square(&self) -> bool {
        return self.width == self.height
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
        height: 12
    };

    let rect2 = Rect {
        width: 11,
        ..rect1
    };

    println!("{:?}", rect1);

    let is_square = rect1.is_square();

    println!("Is rect1 a square?: {}", if is_square {"yes"} else {"no"});
    println!("Can rect1 hold rect2?: {}", if rect1.can_hold(&rect2) {"yes"} else {"no"});
}


fn main() {
    run();
}




