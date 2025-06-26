// Implement Two pointer technique for

// Given a sorted array arr (sorted in ascending order) and a target,
// find if there exists any pair of elements (arr[i], arr[j])
// such that their sum is equal to the target.

pub fn _two_pointer_technique_for_two_sum(sorted_vec: Vec<i8>, target: i8) -> Option<(i8, i8)> {
    let mut left: usize = 0;
    let mut right: usize = sorted_vec.len() - 1;

    while left < right {
        let sum = sorted_vec[left] + sorted_vec[right];

        if sum == target {
            return Some((sorted_vec[left], sorted_vec[right]));
        } else if sum > target {
            right -= 1;
        } else {
            left += 1;
        }
    }

    None
}

pub fn _two_pointer_technique_for_three_sum(
    sorted_vec: Vec<i8>,
    target: i8,
) -> Option<(i8, i8, i8)> {
    for i in 0..sorted_vec.len() - 1 {
        let mut left: usize = i + 1;
        let mut right: usize = sorted_vec.len() - 1;

        let required_sum = target - sorted_vec[i];

        while left < right {
            let new_sum = sorted_vec[left] + sorted_vec[right];

            if required_sum == new_sum {
                return Some((sorted_vec[i], sorted_vec[left], sorted_vec[right]));
            } else if required_sum < new_sum {
                right -= 1;
            } else {
                left += 1;
            }
        }
    }

    None
}

pub fn two_pointer_technique_for_rain_water_trap(unsorted_vec: Vec<i8>) -> i8 {
    let mut water: i8 = 0;
    let mut left: usize = 1;
    let mut right: usize = unsorted_vec.len() - 2;
    let mut l_max = unsorted_vec[left - 1];
    let mut r_max = unsorted_vec[right + 1];

    while left <= right {
        if r_max <= l_max {
            water += (r_max - unsorted_vec[right]).max(0);
            r_max = r_max.max(unsorted_vec[right]);
            right -= 1;
        } else {
            water += (l_max - unsorted_vec[left]).max(0);
            l_max = l_max.max(unsorted_vec[left]);
            left += 1;
        }
    }

    water
}
