use rand::Rng;
use std::cmp::Ordering::Less;
use std::cmp::Ordering::Greater;
use std::cmp::Ordering::Equal;
use std::io;

const FANTASY: f32 = 3.2;

fn guess_number() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Less => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn main() {
    let v = 1;

    println!("Hello, world! {}", v);

    guess_number();
}
