pub fn to_uppercase(current_str: String) -> String {
    current_str
        .chars()
        .map(|ch| ch.to_ascii_uppercase())
        .collect()
}
