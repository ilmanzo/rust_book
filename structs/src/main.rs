#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width*self.height
    }

}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    // let dimensions=(30,50);

    let r = Rectangle {
        width: 30,
        height: 50,
    };
    let a=r.area(); // test for reference borrow
    println!("The area of the rectangle {:?} is {} square pixels", r, r.area());
}


