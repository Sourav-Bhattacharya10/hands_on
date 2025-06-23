pub fn find_common_prefix(words_array: Vec<&str>) -> String {
    let mut result: String = String::from("");
    let mut k: usize = 0;
    let mut i: usize = 0;
    let mut fc: char = 'a';
    let mut flag: bool = true;

    while flag {
        if i == 0 {
            fc = words_array[i].chars().nth(k).unwrap();
        } else if words_array[i].chars().nth(k) == Some(fc) {
            if i == words_array.len() - 1 {
                result += &fc.to_string();
                k += 1;
            }
        } else {
            flag = false;
        }
        i = (i + 1) % words_array.len();
    }

    result
}
