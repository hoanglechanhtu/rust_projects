fn main() {
    println!("Hello, world!");
    let rectangle = (30, 50);
    println!("The area of the rectangle is {}", area(rectangle));
}

fn area(rectangle: (u32, u32)) -> u32 {
    return rectangle.0 * rectangle.1;
}
