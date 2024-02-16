/*
Build a guessing game  
Such that if you input a number 
the game should tell you higher or lower until you reach the number 

PS: Basically, Binary Search on 1-100
*/ 

use std::io; // This is because the IO library isn't generally included in the standard library 

fn main() {
    println!("*** Guess the number ***"); 

    println!("Enter your guess");

    let mut guess = String::new(); // mut because the guess string is mutable 

    io::stdin()
        .read_line(&mut guess) // store it in the guess variable
        .expect("Failed to read the input");  // like a catch block
    
    println!("You guessed: {guess}"); 
}
