fn main() {

    let mut kbd_input_temp= String::new();
    println!("Temperature converter!");
    println!("please input temperature:");
    std::io::stdin().read_line(&mut kbd_input_temp).expect("cannot open standard input");
    //parse the string to a float
    let temp : f32=match kbd_input_temp.trim().parse() {
        Ok(n) => n,
        Err(_) => {panic!("wrong temperature")},
    };
    let conversion=get_temp_symbol();

    let converted= match conversion {
        'C' => convert_to_fahreneit(temp),
        'F' => convert_to_celsius(temp),
        _ => unreachable!()
    };
    println!("{} {} is equivalent to {} {}",temp,conversion,converted,get_reverse_symbol(conversion))

}

fn get_reverse_symbol(s: char) -> char {
    match s {
        'F' => 'C',
        'C' => 'F',
        _ => panic!("wrong symbol")
    }
 
}

fn get_temp_symbol() -> char {
    let mut kbd_input= String::new();
    println!("is this temp in Celsius (C) or Fahreneit (F) ? :");
    std::io::stdin().read_line(&mut kbd_input).expect("cannot open standard input");
    kbd_input.chars().next().unwrap().to_ascii_uppercase()
}

fn convert_to_celsius(tf: f32) -> f32 {
    (tf-32.0)*5.0/9.0
}

fn convert_to_fahreneit(tc: f32) -> f32 {
    tc*9.0/5.0 +32.0
}


