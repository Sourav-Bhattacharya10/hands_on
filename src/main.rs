// mod reverse_string;
// mod factorial;
// mod fibonacci;
// mod balanced_array;
// mod to_uppercase;
mod two_pointer;

// use factorial::factorial;
// use fibonacci::old_fibonacci;
// use reverse_string::reverse_string;
// use balanced_array::is_balanced_array;
// use to_uppercase::to_uppercase;
use two_pointer::two_pointer_technique_for_rain_water_trap;

fn main() {
    // let text = "sourav";
    // println!("Reversed String: {:?}", reverse_string(text));

    // let n: u32 = 5;
    // println!("Factorial of {} is {}", n, factorial(n));

    // let n: u32 = 7;
    // println!("Fibonacci of {}th term is {}", n, old_fibonacci(n));

    // println!("[ -2, 3, 2, -3 ] → {}", is_balanced_array(vec![ -2, 3, 2, -3 ]));
    // println!("[ 1, 1, -1, -1 ] → {}", is_balanced_array(vec![ 1, 1, -1, -1 ]));
    // println!("[ 1, 1, -1 ] → {}", is_balanced_array(vec![ 1, 1, -1 ]));
    // println!("[ -2, 3, 2, -3, 0, 5, -5 ] → {}", is_balanced_array(vec![ -2, 3, 2, -3, 0, 5, -5 ]));
    // println!("[ 1, 2, -3 ] → {}", is_balanced_array(vec![ 1, 2, -3 ]));
    // println!("[ -3, -2, -3, -2, 4, 1, 4, 1, 3, 2, -4, -1 ] → {}", is_balanced_array(vec![ -3, -2, -3, -2, 4, 1, 4, 1, 3, 2, -4, -1 ]));

    // println!("to uppercase : {}", to_uppercase(String::from(text)));

    // println!(
    //     "[ 10, 20, 35, 50 ] → {:?}",
    //     two_pointer_technique_for_two_sum(vec![10, 20, 35, 50], 70i8)
    // );
    // println!(
    //     "[ 10, 20, 30 ] → {:?}",
    //     two_pointer_technique_for_two_sum(vec![10, 20, 30], 70i8)
    // );
    // println!(
    //     "[ 1, 4, 6, 8, 10, 45 ] → {:?}",
    //     two_pointer_technique_for_three_sum(vec![1, 4, 6, 8, 10, 45], 13i8)
    // );
    println!(
        "[ 3, 0, 1, 0, 4, 0, 2 ] → {:?}",
        two_pointer_technique_for_rain_water_trap(vec![3, 0, 1, 0, 4, 0, 2])
    );
}
