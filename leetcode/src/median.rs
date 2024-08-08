pub fn binary_search_on_2_lists(list_1: Vec<i32>, list_2: Vec<i32>) -> f64 {
    if list_1.len() > list_2.len() {
        return binary_search_on_2_lists(list_2, list_1);
    }
    let mut start = 0;
    let mut end = list_1.len();
    let median_pos = (list_1.len() + list_2.len() + 1) / 2;
    while start <= end {
        let mid = (start + end) / 2;
        let left_2_size = median_pos - mid;

        let left_1 = match &mid.checked_sub(1) {
            Some(v) => list_1[*v],
            None => i32::MIN,
        };
        let left_2 = match &left_2_size.checked_sub(1) {
            Some(v) => list_2[*v],
            None => i32::MIN,
        };
        let right_1 = if mid < list_1.len() {
            list_1[mid]
        } else {
            i32::MAX
        };
        let right_2 = if left_2_size < list_2.len() {
            list_2[left_2_size]
        } else {
            i32::MAX
        };
        println!("{left_1} {left_2} {right_1} {right_2}");

        if left_1 <= right_2 && left_2 <= right_1 {
            if (list_1.len() + list_2.len()) % 2 == 0 {
                return (std::cmp::max(left_1, left_2) as f64
                    + std::cmp::min(right_1, right_2) as f64)
                    / 2.0;
            } else {
                return std::cmp::max(left_1, left_2) as f64;
            }
        } else if left_1 > right_2 {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }
    return 0.0;
}
