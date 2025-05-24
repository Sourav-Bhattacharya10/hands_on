// old way
pub fn old_fibonacci(n: u32) -> u32 {
    if n <= 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    old_fibonacci(n - 1) + old_fibonacci(n - 2)
}

// new way in rust
// pub enum Fibonacci {
//     Base(u32),
//     Recursive()
// }
