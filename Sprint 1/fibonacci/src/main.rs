use std::io;

fn main() {
    println!("Welcome to my little Fibonacci Sequence program!");
    println!("For this program, I am asking for you to give me a number and I will print every number in the Fibonacci Sequence between 1 and that number.");

    loop
    {

        // Get user input
        let mut fib = String::new();
        println!("What number shall I count to? If you wish to end the program, type '0'");
    
        io::stdin()
        .read_line(&mut fib)
        .expect("Failed to read line!");
        let fib: usize = fib.trim()
                          .parse()
                          .expect("You must input a number!");

        // End program if user types 0.                  
        if fib == 0
        {
            break;
        }

        // Else, play game.
        else if fib > 0
        {
            let answer = get_answer(fib);

            if answer == 1
            {
                add_sequence(fib)
            }

            else
            {
                add_iterator(fib)    
            }
        }
    }

}


// Iterate until we reach the num user gives.
fn add_sequence(fib: usize)
{
    let mut my_vec = vec![1, 1];
    let mut i: usize = 2;

    while (my_vec[i - 1] + my_vec[i - 2]) <= fib {
        let next_fib = my_vec[i - 1] + my_vec[i - 2];
        my_vec.push(next_fib);
        i += 1;
    }

    println!("Fibonacci sequence up to {}: {:?}", fib, my_vec);
} 


// Iterate as many times as the urser specifys
fn add_iterator(fib: usize)
{
    let mut my_vec = vec![1, 1];
    let mut i: usize = 2;
    let mut iterator = my_vec.len() + 1;

    while iterator <= fib {
        let next_fib = my_vec[i - 1] + my_vec[i - 2];
        my_vec.push(next_fib);
        i += 1;
        iterator += 1;
    }

    println!("Fibonacci sequence up to {}: {:?}", fib, my_vec);
}


// Ask user if they want to count to 10 or iterate 10 times.
fn get_answer(fib: usize) -> u64 {
    loop {
        let mut answer = String::new();
        println!("Would you like to count every number between 1 and {} or count until we iterate {} time(s)", fib, fib);
        println!("Type 1 to count to {}", fib);
        println!("Type 2 to iterate {} time(s)", fib);

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line!");

        let answer = match answer.trim().parse::<u64>() 
        {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid option.");
                continue;
            }
        };

        if answer == 1 || answer == 2 
        {
            return answer;
        } 
        
        else 
        {
            println!("Not an available option.");
        }
    }
}
