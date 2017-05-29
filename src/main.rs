extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Random number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Counter variable in order to track the used tries.
    let mut tries: i32 = 0;

    // Infinite loop.
    loop {
        println!("Please input your guess: ");

        // Variable of boxed String type. The value will
        // be stored on the heap, since its size is not fixed.
        let mut guess = String::new();

        // Read the input and save it to the provided
        // borrowed variable.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Trim the input and convert the String type to an
        // integer type, in this case u32. Assign the number
        // or continue the loop if no valid input is given.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Increment count.
        tries += 1;
        println!("You guessed: {}", guess);

        // Compare the guess with the secret generated number
        // and act accordingly.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("You took {} tries to guess the number!", tries);
                break;
            }
        }
    }
}
