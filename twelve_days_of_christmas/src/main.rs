fn print_up_to(n: usize) {
    const DAYS: [&str;12]= [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "10th", "11th", "12th",
    ];

    const GIFTS: [&str;12] = [
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
        DAYS[n-1]
    );
    for d in (0..n).rev() {
        println!("{}", GIFTS[d]);
    }
}

fn main() {
    for day in 1..13 {
        print_up_to(day)
    }
}
