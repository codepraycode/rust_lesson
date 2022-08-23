use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    println!("The Secret number is {}", secret_number);

    loop{
        println!("Please input your guess: ");

        let mut guess: String =  String::new();


        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You Guessed: {}", guess );


        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}","Too Small!".red()),
            Ordering::Greater => println!("{}","Too big".red()),
            Ordering::Equal => {
                println!("{}","You got it!".green());
                break;
            },
        }
    }

    

}
