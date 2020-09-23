extern crate rand;

use rand::thread_rng;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret_number = thread_rng().gen_range(1, 101);

    

    println!("Your secret number is {}", secret_number);
    loop {
        let mut keyboard_input = String::new();
        println!("please input your number");
        std::io::stdin()
            .read_line(&mut keyboard_input)
            .expect("Failed to read line");
        

        let guess: u32 = match keyboard_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Equal => { println!("You win!"); break} ,
            Ordering::Greater => println!("too big!"),
        }
    }
}
