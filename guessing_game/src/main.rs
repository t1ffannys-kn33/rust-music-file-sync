use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is {}", secret_number);
    println!("Hello, world!");
    println!("guessing game, please guess your number");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed to read line!!!");

    let guess: u32 = guess.trim().parse().expect("plz type a number");

    println!("you guessed:{}", guess);
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("to small"),
      Ordering::Equal => println!("u win!"),
      Ordering::Greater => println!("too big")
    }
}
