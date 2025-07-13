mod kadanes;

use kadanes::max_subarray_sum;

fn main() {
    let arr: Vec<i8> = vec![2, 3, -8, 7, -1, 2, 3];
    println!(
        "Kadane's algorithm max subarray sum: {}",
        max_subarray_sum(arr)
    );
}
