use  std::io;            // This line allows me to get user input.
use std::cmp::Ordering; // This will give us comparitors such at '=>' and '<='
use rand::Rng;         // This line allows us to use the rand library.

// fn means function. Use it to let Rust know that we are defining a function.
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // thread_rnd() allows us to use a specific number generator that is local to this program
                                                                    // gen_range () generates a range between a specified amount.

    println!("The secret number is: {secret_number}");

    loop{
        println!("Please input a number");

        /*  
            We use 'let' to declare an unchanging variable. 
            We use 'mut' to let the program know that the variable can change. mut stands for 'mutability'.
            We must use String::new() to let the program know that the variable will be a string.
        */
        let mut guess = String::new();

        /* 
            The next two lines is how we read from the user input.
            The next line over is exception handling in case the input it unreadable.
        */
        io::stdin() // This line could be rewritten as "std::io::stdin" if we didn't have "use  std::io" already written.
            .read_line(&mut guess) // The "&" means that we are passing in a refrence as an argument. This prevents our data from being overwritten.
            .expect("Failed to read line!");

        // The line below is how we parse a string to an int. trim method removes white space. parse method converts string to int.
        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Just right");
                break;
            }
        }
    }
}
