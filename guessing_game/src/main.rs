use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("secret number: {}", secret_number);


    loop {
        let mut guess = String::new();
        println!("Please input your guess");

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("less!"),
            Ordering::Greater => println!("greater!"),
            Ordering::Equal => {
                println!("correct!");
                break;
            },
        }
    }
}
