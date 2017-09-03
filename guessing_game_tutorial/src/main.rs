extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut win = false;
    println!("Guess a number");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("failed to read line");
    let answer = rand::thread_rng().gen_range(1,11);
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");
    match guess.cmp(&answer){
        Ordering::Less => println!("too low"),
        Ordering::Greater => println!("too high"),
        Ordering::Equal => win = true,
    }

    if !win {
        main();
    } else{
        println!("you guessed the number! you won!");
    }
}
