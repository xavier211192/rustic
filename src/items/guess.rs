use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    //Guessing game
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("Secret number is {secret_number}");

    loop {
        println!("Enter your guess!");
        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("failed to read");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
