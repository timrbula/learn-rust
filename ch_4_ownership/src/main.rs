fn main() {
    basic();
    new_tings();
    let s = create_and_return();
    recieve_and_print(&s);
    print_helper("Main:", s);
    duplicate_ref();

    let num: usize = 2;
    make_copy(num);
    print_helper("Main:", num);

    try_to_mutate();

    let sentence = String::from("My Name is Fred");
    let fst_wrd = first_word(&sentence);
    print_helper("Main:", fst_wrd);
}

fn basic() {
    println!("Basic:");
    let s = "hello";
    {
        let s = "world";
        let wo = &s[0..2];
        println!("{}", s);
        println!("{}", wo);
    }
    println!("{}", s);
    println!();
}

fn new_tings() {
    println!("New Tings:");
    let mut s = String::new();
    let b = String::new();
    println!("{}", b == s);
    println!("{}", &b.eq(&s));
    s.push('a');
    println!("{}", s);
    s.push('b');
    println!("{}", s);
    s.push_str("cdef");
    println!("{}", s);
    println!();
}

fn create_and_return() -> String {
    let s = String::from("create");
    return s;
}

fn recieve_and_print(s: &String) {
    print_helper("Recieve and Print:", s);
}
fn duplicate_ref() {
    println!("Move Ref:");
    let mut s1 = String::from("you again");
    println!("{}", s1);
    s1.push_str(", what?");
    let mut s2 = s1;
    s2.push_str(", what?");
    println!("{}", s2);
    println!();
}

fn make_copy(int: usize) {
    print_helper("Make Copy:", int);
}

fn try_to_mutate() {
    let mut s = String::from("hello");
    let t = &mut s;
    t.push_str("a");
    let mut clone = t.clone();
    clone.push_str("what");
    print_helper("Try To Mutate:", clone);
}

fn print_helper<T>(title: &str, value: T)
where
    T: std::fmt::Display,
{
    println!("{}", title);
    println!("{}", value);
    println!();
}

fn first_word(s: &str) -> &str {
    let str_iter = s.chars();
    for (index, item) in str_iter.enumerate() {
        if item == ' ' {
            return &s[0..index];
        }
    }

    let return_val = s;
    return return_val;
}
