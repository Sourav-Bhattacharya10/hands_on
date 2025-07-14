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

pub struct Fibonacci {
    a: u32,
    b: u32,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self { a: 0, b: 1 }
    }

    pub fn nth_term(&self, n: u32) -> u32 {
        let result: u32 = match n {
            0 => self.a,
            1 => self.b,
            _ => self.nth_term(n - 1) + self.nth_term(n - 2),
        };

        result
    }
}
