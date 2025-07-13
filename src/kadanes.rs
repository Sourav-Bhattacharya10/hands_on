pub fn max_subarray_sum(arr: Vec<i8>) -> i8 {
    let mut result = arr[0];
    let mut max_ending = arr[0];

    for i in 1..arr.len() {
        let sum = max_ending + arr[i];
        max_ending = sum.max(arr[i]);

        result = result.max(max_ending);
    }

    result
}
