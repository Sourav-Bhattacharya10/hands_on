// using recursion
pub fn reverse_string(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }
    let mut chars = text.chars();
    let first_char = chars.next().unwrap();
    let rest: String = chars.collect();
    format!("{}{}", reverse_string(&rest), first_char)
}
