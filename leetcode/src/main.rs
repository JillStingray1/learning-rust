//! this contains rust solutions to all leetcode problems
mod median;
fn main() {
    let list_1 = vec![1, 3];
    let list_2 = vec![2, 4];
    println!("{}", median::binary_search_on_2_lists(list_1, list_2));
}
