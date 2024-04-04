use std::io;
use rand::Rng;
use core::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess!");
    
    loop {
    let mut guess = String::new();

    io::stdin()
         .read_line(&mut guess)
         //  the read_line method on the standard input handle to get input from the user.
         // full job of read_line is to take whatever the user types into standard input and append that into a string
         .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
    
    println!("you guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("{}", "Too small!".red()),
        Ordering::Greater => println!("{}", "Too Big".red()),
        Ordering::Equal => {
            println!("{}","You win".green());
           break;
           },
    }
}

}
