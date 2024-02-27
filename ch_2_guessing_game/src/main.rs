use rand::Rng;
use std::io;

fn main() {
    let random_number = rand::thread_rng().gen_range(0..=100);
    println!("Guess a number!");
    println!("Def not {random_number}...");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("something went wrong!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\"{}\" is not a number silly!", guess.trim());
                continue;
            }
        };

        match guess.cmp(&random_number) {
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Greater => println!("Too big"),
            std::cmp::Ordering::Equal => println!("Right on, brother"),
        }

        if guess == random_number {
            println!("You win!");
            break;
        }

        println!("Guess again!");
    }
}
