// generic struct for a point which can support multiple data types

struct Point<T> {
    _x: T,
    _y: T,
}
// generic implementation syntax
impl<T> Point<T> {
    pub fn new(_x: T, _y: T) -> Self {
        Point { _x, _y }
    }
}

pub fn example() {
    let _x = Point::new(7.0, 8.0);
    println!("{}", largest(&['a', 'b', 'c']));
    println!("{}", largest(&[1, 2, 3]));
}

// generic function to find largest in list
// we need to limit the types to the ones that are orderable
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
