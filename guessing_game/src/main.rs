use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    // mut makes the variable mutable

    loop {
        let mut guess: String = String::new();
        // if we didn't import `use std::io` at the top, we could've done `std::io::stdin`
        io::stdin()
            // `&` indicates a reference to the memory, references are IMMUTABLE by default or `&mut` to make them mutable
            .read_line(&mut guess)
            .expect("Failed to Read line");

        // We are shadowing the previous `guess` variable
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_)=> continue,
        };

        println!("You guessed: {guess}");

        println!("The secret number is {secret_number}");
        match guess.cmp(&secret_number) {
            // each of these is an `arm`
            // `pattern` => what to do next
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
