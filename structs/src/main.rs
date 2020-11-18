#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
            || self.width >= other.height && self.height >= other.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle{width: size, height: size}
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    // let dimensions=(30,50);

    let r1 = Rectangle {
        width: 30,
        height: 50,
    };

    let r2 = Rectangle {
        width: 40,
        height: 20,
    };

    let a = r1.area(); // test for reference borrow
    println!(
        "The area of the rectangle1 {:?} is {} square pixels",
        r1,
        r1.area()
    );

    println!(
        "The area of the rectangle2 {:?} is {} square pixels",
        r2,
        r2.area()
    );


    if r1.can_hold(&r2) {
        println!("r1 can hold r2")
    } else {
        println!("r1 cannot hold r2")
    }
}
