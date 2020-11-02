struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    // let dimensions=(30,50);

    let r = Rectangle {
        width: 30,
        height: 50,
    };
    let a=area(&r);
    println!("The area of the rectangle is {} square pixels", area(&r));
}

fn area(r: &Rectangle) -> u32 {
    r.height * r.width
}
