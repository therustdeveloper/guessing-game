use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(_) => {
                let guess: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("** Please enter a valid number **\n");
                        continue
                    }
                };

                match guess.cmp(&secret_number) {
                    Ordering::Less => println!("Too small!\n"),
                    Ordering::Greater => println!("Too big!\n"),
                    Ordering::Equal => {
                        println!("You win!");
                        break;
                    },
                }
            }
            Err(e) => println!("error: {e}")
        }
    }
}
