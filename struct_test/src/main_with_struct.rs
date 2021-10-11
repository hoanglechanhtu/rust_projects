struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    println!("Hello, world!");
    let rectangle = Rectangle{
        width: 30,
        height: 50,
    };
    println!("The rectangle is {:?}", rectangle);
    println!("The area of the rectangle is {}", area(rectangle));
}

fn area(rectangle: Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}
