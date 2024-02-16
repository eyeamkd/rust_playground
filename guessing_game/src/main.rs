/*
Build a guessing game  
Such that if you input a number 
the game should tell you higher or lower until you reach the number 

PS: Basically, Binary Search on 1-100
*/ 

use std::io;

fn main() {
    println!("*** Guess the number ***"); 

    println!("Enter your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the input"); 
    
    println!("You guessed: {guess}");
}
