pub fn example() {
    closures();
    iterators();
}

pub fn closures() {
    // closures allow us to define a function that returns
    // a variable in place
    let mut x: Result<i32, i32> = Ok(3);
    println!("{}", x.unwrap());
    x = Err(37);
    // here the value to be printed is computed by passing a closure to unwrap_or_else
    // the value error is taken from it's environment, where error here reprents
    // the value inside of x
    println!("{}", x.unwrap_or_else(|error| error));
}

pub fn iterators() {
    // this is how we create an iterator
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let v2_iter = v1_iter.map(|x| x + 1);
    for x in v2_iter {
        println!("{x}");
    }
}
