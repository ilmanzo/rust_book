fn print_up_to(n: usize) {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "10th", "11th", "12th",
    ];

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "10 lords a-leaping",
        "11 pipers piping",
        "12 drummers drumming",
    ];
    println!();
    println!(
        "on the {} day of Christmas my true love sent to me",
        days[n-1]
    );
    for d in (0..n).rev() {
        println!("{}", gifts[d]);
    }
}

fn main() {
    for day in 1..13 {
        print_up_to(day)
    }
}
