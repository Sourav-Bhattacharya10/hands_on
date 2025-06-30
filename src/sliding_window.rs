use std::collections::{HashMap, HashSet};

pub fn basic_sliding_window(arr: Vec<i8>, k: usize) -> i8 {
    let n = arr.len();

    let mut max_sum: i8 = i8::MIN;

    if k > n {
        panic!("k cannot be greater than the array length.");
    } else {
        let mut current_sum = 0;
        for i in 0..k {
            current_sum += arr[i];
        }

        max_sum = max_sum.max(current_sum);

        for i in k..arr.len() {
            current_sum += arr[i] - arr[i - k];
            max_sum = max_sum.max(current_sum);
        }
    }

    max_sum
}

pub fn longest_subarray(arr: Vec<i8>, k: i8) -> usize {
    let mut result: usize = 0;
    let mut prefix_sum: i8 = 0;
    let mut mp: HashMap<i8, usize> = HashMap::new();

    for i in 0..arr.len() {
        prefix_sum += arr[i];
        let extra_sum: i8 = prefix_sum - k;

        if prefix_sum == k {
            result = i + 1;
        } else if mp.contains_key(&extra_sum) {
            result = result.max(i - mp.get(&extra_sum).unwrap());
        }

        if !mp.contains_key(&arr[i]) {
            mp.insert(arr[i], i);
        }
    }

    result
}

pub fn length_of_longest_substring(s: &str) -> usize {
    let mut max_length: usize = 0;
    let mut left: usize = 0;
    let mut right: usize = 0;
    let mut char_set: HashSet<char> = HashSet::new();
    let s_chars: Vec<char> = s.chars().collect();

    while right < s_chars.len() {
        while char_set.contains(&s_chars[right]) {
            char_set.remove(&s_chars[left]);
            left += 1;
        }

        char_set.insert(s_chars[right]);

        max_length = max_length.max(right - left + 1);

        right += 1;
    }

    return max_length;
}
