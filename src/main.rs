use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    
    let secret_number = rand::thread_rng().gen_range(1..101); 
    
    println!("Guess the number!\n");

    let mut found = false;
    
    while(!found){

    println!("Please input your guess.\n");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!") ;
            found = true;
        }
    }
    }
    
}
