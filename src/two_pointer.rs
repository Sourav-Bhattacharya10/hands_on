// Implement Two pointer technique for

// Given a sorted array arr (sorted in ascending order) and a target,
// find if there exists any pair of elements (arr[i], arr[j])
// such that their sum is equal to the target.

pub fn two_pointer_technique(sorted_vec: Vec<i8>, target: i8) -> (bool, i8, i8) {
    let mut result = false;

    let mut left: usize = 0;
    let mut right: usize = sorted_vec.len() - 1;

    let mut num1: i8 = -1;
    let mut num2: i8 = -1;

    while left < right {
        num1 = sorted_vec[left];
        num2 = sorted_vec[right];
        let sum = num1 + num2;
        if sum == target {
            result = true;
            break;
        } else if sum > target {
            right -= 1;
        } else {
            left += 1;
        }
    }

    (result, num1, num2)
}
