use std::cmp::Ordering;

fn ask_secret_number() -> u32 {
    let mut user_input = String::new();
    println!("please enter a number (1-99)");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("cannot read stdin");

    match user_input.trim().parse() {
        Ok(n) => n,
        Err(_) => 0,
    }
}

fn main() {
    println!("welcome to reverse guessing!");

    let mut secret_number = 0;
    while secret_number <= 0 && secret_number < 100 {
        secret_number = ask_secret_number();
    }
    println!("the secret number I will guess is: {}", secret_number);

    let mut min = 1;
    let mut max = 100;

    loop {
        let guess = (min + max) / 2;
        println!("my guess: {}   min={}  max={}", guess, min, max);
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("found it: {}", guess);
                break;
            }
            Ordering::Less => min = guess,     // if we are low, raise the minimum
            Ordering::Greater => max = guess,  // if we are high, lower it
        }
    }
}
