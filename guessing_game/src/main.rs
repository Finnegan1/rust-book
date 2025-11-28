use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number {secret_number}!");

    loop {
        println!("Please enter a number.");
        let mut guess = String::new();

        let res: Result<usize, io::Error> = io::stdin().read_line(&mut guess);
        res.expect("");

        println!("Du hast geschätzt: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "too small".red()),
            Ordering::Greater => println!("{}", "too big".red()),
            Ordering::Equal => {
                println!("{}", "Selected correct answer".green());
                break;
            }
        }
    }
}
