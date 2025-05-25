use std::collections::hash_map::HashMap;

pub fn is_balanced_array(input_arr: Vec<i8>) -> bool {
    let result;
    let mut count_hash_map: HashMap<i8, i8> = HashMap::new();

    for val in input_arr {
        if val == 0 {
            continue;
        }
        else if val > 0 {
            count_hash_map.insert(val, count_hash_map.get(&val).unwrap_or(&0) + 1);
        }
        else {
            count_hash_map.insert(val.abs(), count_hash_map.get(&val.abs()).unwrap_or(&0) - 1);
        }
    }

    result = count_hash_map.values().all(|val| *val == 0);

    result
}