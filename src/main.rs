// mod reverse_string;
// mod factorial;
mod fibonacci;

// use factorial::factorial;
use fibonacci::old_fibonacci;
// use reverse_string::reverse_string;

fn main() {
    // let text = "sourav";
    // println!("Reversed String: {:?}", reverse_string(text));

    // let n: u32 = 5;
    // println!("Factorial of {} is {}", n, factorial(n));

    let n: u32 = 7;
    println!("Fibonacci of {}th term is {}", n, old_fibonacci(n));
}
