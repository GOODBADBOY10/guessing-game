use std::cmp::Ordering;
use std::io;
// to obtain a user input and also print it out, io is used which is from the standard libary
use rand::Rng;
// this is used to generate random numbers

fn main() {
    loop {
        println!("Guessing number game");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("The secret number is: {secret_number}");

        println!("Enter a number to guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            // to get input from the user
            // references are immutable by default
            .expect("You failed to guess any number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // let guess: u32 = if let Ok(num) = guess.trim().parse() {
        //     num
        // } else {
        //     continue;
        // };

        println!("You guessed: {guess} ");

        // the cmp method compares too values
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// use std::cmp::Ordering;
// use std::io;

// use rand::Rng;

// fn main() {
//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     loop {
//         println!("Please input your guess.");

//         let mut guess = String::new();

//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         println!("You guessed: {guess}");

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }
