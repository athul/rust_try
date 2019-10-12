use std::io;
use rand::Rng;

fn main(){
println!("Guess the Number");
let secret_number = rand::thread_rng().gen_range(0,101);

let mut guess=String::new();
println!("What's your Guess?");
io::stdin().read_line(&mut guess)
    .expect("Failed to Read Line");

println!("The secret number is {}",secret_number);
println!("You guessed:{}",guess);
}
