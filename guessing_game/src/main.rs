/*
    io: input/output library
    `io` library comes from the standard library, `std`
*/
use std::io; // for into scope
use std::cmp::Ordering; // Ordering is a enum
use rand::Rng;

fn main() {
    println!("Guess the number");

    /*
        - rand::thread_rng > particular random number generator
        - gen_range > is defined by the `Rng`, takes a range expression and generates a random number in the range
        - start..=end > inclusive on the lower and upper bounds ex.1..=100 means from 1 to 100
    */
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {

        println!("Please input your guess");

        // --- Storing Values with Variables ---

        /*
        [Declaration variables]
            In Rust, variables are immutable by default.
            When use `mut`, that varibale will be mutable
            let apples = 5; // immutable
            let mut bananas = 5; // mutable

        [String::new()]
            String::new() returns a new instance of a String.
            ::new : syntax is an associated function of the String type in Rust
        */
        
        // ---  Receiving User Input --- 
        
        let mut guess = String::new();
    

        io::stdin() // use stdin function from id
            /*
                - getting input from the user (variable mut guess)
                - & indicates this argument is a reference.
                - If you don’t call expect, the program will compile, but you’ll get a warning: Unused Wrarning
            */
            .read_line(&mut guess) 
            .expect("Failed to read line");

        // Shadowing: reuse the variable name
        /* parse(): 
            - convert String to Number as typed :u32
            - only work on characters can be logically converted into numbers (except emojis)
            - return a Return type
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // tells the program to go to the next iteration of the loop
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) { // compare guess to secret_number
            // using enum Ordering `Less`,`Greater`,`Equal`
            Ordering::Less => println!("Too small!"), 
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // exit loop
            }
        }
    }
}
