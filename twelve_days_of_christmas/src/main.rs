fn print_up_to(n: usize) {
    let lyrics = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "five gold rings",
        "Six geese a laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "10 lords a-leaping",
        "11 pipers piping",
        "12 drummers drumming",
    ];
    println!();
    println!("on the {} day of Christmas my true love sent to me", n);
    for d in (0..n).rev() {
        println!("{}", lyrics[d]);
    }
}

fn main() {
    for day in 1..13 {
        print_up_to(day)
    }
}
