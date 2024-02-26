/*
Build a guessing game  
Such that if you input a number 
the game should tell you higher or lower until you reach the number 

PS: Basically, Binary Search on 1-100
*/

use std::io; // This is because the IO library isn't generally included in the standard library
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("*** Guess the number ***");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut tries = 0;
    loop {
        println!("Enter your guess");
        let mut guess = String::new(); // mut because the guess string is mutable

        io::stdin()
            .read_line(&mut guess) // store it in the guess variable
            .expect("Failed to read the input");  // like a catch block

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_)=>continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("You guessed it right");
                break;
            },
            Ordering::Greater=> println!("Too Big")
        }
        tries+=1;
    }
    println!("You took {tries} Tries");
}
