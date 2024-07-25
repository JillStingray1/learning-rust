// defines the Struct Rectangle
struct Rectangle {
    width: u32,
    height: u32,
}

// defines methods for Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Example of how to construct a rectangle
fn build_rectangle(height: u32, width: u32) -> Rectangle {
    Rectangle { width, height }
}

pub fn example() {
    let x = build_rectangle(2, 10);
    println!("{}", x.area())
}
