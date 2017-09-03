extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess a number");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("failed to read line");
    let answer = rand::thread_rng().gen_range(1,11);
    println!("You guessed: {} and the number was {}", guess, answer);
}
