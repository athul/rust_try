use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        println!("Guess the Number");
        let secret_number = rand::thread_rng().gen_range(0, 101);

        let mut guess = String::new();
        println!("What's your Guess?");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to Read Line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("--------Less"),
            Ordering::Greater => println!("-------You're a rocket boi!!!"),
            Ordering::Equal => {
                println!("--------Yay You're good");
                break;
            }
        }
        println!("The secret number is {}", secret_number);
        println!("You guessed:{}", guess);
    }
}
