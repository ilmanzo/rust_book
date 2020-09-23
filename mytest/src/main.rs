fn main() {
    let mut keyboard_input = String::new();
    loop {
        std::io::stdin().read_line(&mut keyboard_input).expect("oops");
        println!("You entered: {}", keyboard_input);
    }
}
