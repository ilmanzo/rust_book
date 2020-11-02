fn main() {
    // let width1 = 30;
    // let height1 = 50;

    let dimensions=(30,50);

    println!(
        "The area of the rectangle is {} square pixels",
        area(dimensions)
    );
}

fn area(dim:(u32,u32)) -> u32 {
    dim.0 * dim.1
}
