fn main() {
    let hello="hello, world";

    let mut hs=&hello[3..6];


    hs="abc";
    println!("{}",hello)
}
