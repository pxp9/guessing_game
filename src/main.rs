use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn compare_guess(guess: u32, secret_number: u32) -> bool {
    println!("You guessed: {}", guess);
    let mut found = false;
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            found = true;
        }
    }
    found
}

fn read_number() -> String {
    println!("Please input a number to guess.\n");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line\n");
    guess
}
fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    println!("Guess the number!\n");

    let mut found = false;
    let mut guess: String = read_number();
    let func = |i: i32| -> i32 { i + 1 };
    while !found {
        match guess.trim().parse::<u32>() {
            Ok(guess_number) => {
                found = compare_guess(guess_number, secret_number);
                if !found {
                    guess = read_number();
                }
            }
            Err(_e) => guess = read_number(),
        }
    }
}
